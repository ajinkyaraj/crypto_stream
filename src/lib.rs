pub mod binance;
pub mod bitstamp;
mod common;

use futures_util::{StreamExt};

pub async fn connect_web_stream(symbol: &str)-> Result<(), Box<dyn std::error::Error>> {

   let (_, mut binance_read) = binance::connect(symbol).await.expect("Could not connect to binance").split();
   let (_, mut bitstamp_read) = bitstamp::connect(symbol).await.expect("Could not connect to bitstamp").split();
    loop {
        let msg = tokio::select! {
             Some(msg) = binance_read.next() => {println!("Binance Msg:"); binance::decode(&msg.unwrap())},
             Some(msg) = bitstamp_read.next() => { println!("Bitstamp msg:"); bitstamp::decode(&msg.unwrap())},
             else => {
                    println!("No more messages");
                    break;
                },
         };
    }
    Result::Ok(())
}