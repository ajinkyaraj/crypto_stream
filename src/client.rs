use std::env;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use orderbook::orderbook_aggregator_client::OrderbookAggregatorClient;

mod orderbook {
    tonic::include_proto!("orderbook");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a port to subscribe to ,default:  50001");
    }
    let port: i32 = args[1].parse().expect("Please enter a valid port");
    let addr = format!("http://[::1]:{}", port);

    let mut client = OrderbookAggregatorClient::connect(addr).await?;
    let request = tonic::Request::new(orderbook::Empty {});

    let mut response = client.book_summary(request).await?.into_inner();

    // setting up indicatif
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_spinner().template(
        "{prefix:.bold} {spinner} {bar:20.red/blue} {wide_msg}").tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let bids_bars: Vec<ProgressBar> = vec![
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
    ];

    let spread_bar
        = m.add(ProgressBar::new(100));

    let asks_bars: Vec<ProgressBar> = vec![
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
        m.add(ProgressBar::new(100)),
    ];


    spread_bar
        .set_prefix(format!("[Spread]"));
    spread_bar.set_style(sty.clone());
    bids_bars.iter()
        .enumerate()
        .for_each(|(i, pb)| {
            pb.set_prefix(format!("[Bid  {}]", i.abs_diff(9)));
            pb.set_style(sty.clone());
        });
    asks_bars.iter()
        .enumerate()
        .for_each(|(i, pb)| {
            pb.set_prefix(format!("[Ask  {}]", i));
            pb.set_style(sty.clone());
        });

    tokio::spawn(async move { let _ = m.join_and_clear(); });

    // listening to stream
    while let Some(summary) = response.message().await? {
        let result: orderbook::Summary = summary;
        let spread = result.spread;
        let bids = result.bids;
        let asks = result.asks;

        let perc = (spread / asks[0].price) * 100.0;
        spread_bar.set_message(format!("{} ({}%)", spread
                                       , perc));
        bids.iter().enumerate().for_each(|(i, level)|
            {
                bids_bars[i].set_message(format!("{} {} {}", level.price, level.amount, level.exchange));
                let pos = level.amount as u64;
                bids_bars[i].set_position(pos);
            }
        );
        asks.iter().enumerate().for_each(|(i, level)|
            {
                asks_bars[i].set_message(format!("{} {} {}", level.price, level.amount, level.exchange));
                let pos = level.amount as u64;
                asks_bars[i].set_position(pos);
            }
        );
    }

    Ok(())
}