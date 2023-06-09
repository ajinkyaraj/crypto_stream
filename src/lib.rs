use std::collections::HashMap;
use std::sync::Arc;

use futures_util::StreamExt;
use tokio::sync::{RwLock, watch};
use tonic::transport::Server;

use server::orderbook::{Level, Summary};
use server::orderbook;

use crate::common::*;
use crate::server::OrderbookService;

pub mod binance;
pub mod bitstamp;
pub mod common;
pub mod server;


pub(crate) type SummaryChannel = (watch::Sender<Summary>, watch::Receiver<Summary>);

pub async fn connect_web_stream(symbol: &str, port: i32) -> Result<(), Box<dyn std::error::Error>> {
    // create a BTreeMap named top_20_bids  with key, value pair of OrderedFloat and Level , sorted by  descending order of
    let summary_ch = Arc::new(RwLock::new(watch::channel(Summary {
        spread: Default::default(),
        bids: vec![],
        asks: vec![],
    })));
    let summary_ch_main = summary_ch.clone();
    let orderbook_service = OrderbookService::new(summary_ch);

    tokio::spawn(async move {
        let addr = format!("[::1]:{}", port);
        let addr = addr.parse().expect("Could not parse address");
        Server::builder()
            .add_service(orderbook::orderbook_aggregator_server::OrderbookAggregatorServer::new(orderbook_service))
            .serve(addr)
            .await.expect("Could not start server");
    });

    let mut exchange_quotes: HashMap<Exchanges, TopQuotesWithExchange> = HashMap::new();
    let mut exchange_index: HashMap<Exchanges, (usize, usize)> = HashMap::from([(Exchanges::Binance, (0, 0)), (Exchanges::Bitstamp, (0, 0))]);
    let (_, mut binance_read) = binance::connect(symbol).await.expect("Could not connect to binance").split();
    let (_, mut bitstamp_read) = bitstamp::connect(symbol).await.expect("Could not connect to bitstamp").split();
    loop {
        let msg = tokio::select! {
Some(msg) = binance_read.next() => {  binance::decode( & msg.unwrap())},
Some(msg) = bitstamp_read.next() => { bitstamp::decode( & msg.unwrap())},
else => {
println ! ("No more messages");
break;
},
};
        match msg {
            Ok(msg) =>
                {
                    exchange_quotes.remove(&msg.exchange);
                    exchange_quotes.insert(msg.exchange, msg);
                }
            Err(_) => continue,
        }
        let mut summary = Summary { spread: 0f64, bids: vec![], asks: vec![] };
        exchange_index.iter_mut().for_each(|(_, v)| *v = (0, 0));
        for _i in 0..10usize {
// get top 10 bids
            let mut max_bid = Quotes { price: 0f64, amount: 0f64 };
            let mut max_bid_exchange = Exchanges::Binance;
            let mut min_ask = Quotes { price: f64::MAX, amount: 0.0f64 };
            let mut min_ask_exchange = max_bid_exchange;
            for (k, v) in exchange_index.iter_mut() {
                if !exchange_quotes.contains_key(k) { continue; }
                if (max_bid.price - exchange_quotes[k].quotes.bids[v.0].price).abs() < 1e-9 {
                    if max_bid.amount < exchange_quotes[k].quotes.bids[v.0].amount {
                        max_bid = exchange_quotes[k].quotes.bids[v.0];
                        max_bid_exchange = *k;
                    }
                } else if max_bid.price < exchange_quotes[k].quotes.bids[v.0].price {
                    max_bid = exchange_quotes[k].quotes.bids[v.0];
                    max_bid_exchange = *k;
                }
                if (min_ask.price - exchange_quotes[k].quotes.asks[v.1].price).abs() < 1e-9 {
                    if min_ask.amount < exchange_quotes[k].quotes.asks[v.1].amount {
                        min_ask = exchange_quotes[k].quotes.asks[v.1];
                        min_ask_exchange = *k;
                    }
                } else if min_ask.price > exchange_quotes[k].quotes.asks[v.1].price {
                    min_ask = exchange_quotes[k].quotes.asks[v.1];
                    min_ask_exchange = *k;
                }
            }
            summary.bids.push(Level { exchange: exchange_to_str(max_bid_exchange), price: max_bid.price, amount: max_bid.amount });
            summary.asks.push(Level { exchange: exchange_to_str(min_ask_exchange), price: min_ask.price, amount: min_ask.amount });
// Increment index which were found to be best bid or best ask.
            exchange_index.entry(max_bid_exchange).and_modify(|x| x.0 += 1);
            exchange_index.entry(min_ask_exchange).and_modify(|y| y.1 += 1);
        }
        summary.spread = summary.asks[0].price - summary.bids[0].price;
        summary_ch_main.write().await.0.send(summary).expect("Disconnected from the server.");
    }
    Ok(())
}