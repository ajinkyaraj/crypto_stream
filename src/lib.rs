pub mod binance;
pub mod bitstamp;
use futures_util::{StreamExt};

pub async fn connect_web_stream(symbol: &str)-> Result<(), Box<dyn std::error::Error>> {

   let (_, mut binance_read) = binance::connect_binance(symbol).await.expect("Could not connect to binance").split();
   let (_, mut bitstamp_read) = bitstamp::connect_bitstamp(symbol).await.expect("Could not connect to bitstamp").split();
    loop {
        let msg = tokio::select! {
             Some(msg) = binance_read.next() => {println!("Binance Msg:"); msg},
             Some(msg) = bitstamp_read.next() => { println!("Bitstamp msg:"); msg},
             else => {
                    println!("No more messages");
                    break;
                },
         };
        println!("{:?}", msg);
    }
    Result::Ok(())
}