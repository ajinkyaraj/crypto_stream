use std::collections::{BTreeMap, hash_map};

use futures_util::StreamExt;

use ordered_float::OrderedFloat;

use crate::common::{Exchanges, Level};

pub mod binance;
pub mod bitstamp;
mod common;

pub async fn connect_web_stream(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    // create a BTreeMap named top_10_bids  with key, value pair of OrderedFloat and Level , sorted by  descending order of keys
    let mut top_10_bids: BTreeMap<OrderedFloat<f64>, Level> = BTreeMap::new();
    let mut top_10_asks: BTreeMap<OrderedFloat<f64>, Level> = BTreeMap::new();

    let (_, mut binance_read) = binance::connect(symbol).await.expect("Could not connect to binance").split();
    let (_, mut bitstamp_read) = bitstamp::connect(symbol).await.expect("Could not connect to bitstamp").split();
    loop {
        let msg = tokio::select! {
             Some(msg) = binance_read.next() => { (Exchanges::Binance, binance::decode(&msg.unwrap()))},
             Some(msg) = bitstamp_read.next() => { (Exchanges::Bitstamp, bitstamp::decode(&msg.unwrap()))},
             else => {
                    println!("No more messages");
                    break;
                },
         };
        match msg {
            (exch, Ok(msg)) => {
                for bid in msg.bids {
                    top_10_bids.insert(OrderedFloat(bid.price), Level { exchange: exch, price: bid.price, amount: bid.amount });
                }
                for ask in msg.asks {
                    top_10_asks.insert(OrderedFloat(ask.price), Level { exchange: exch, price: ask.price, amount: ask.amount });
                }
                loop {
                    if top_10_bids.len() < 11 { break; }
                    top_10_bids.pop_first();
                }
                loop {
                    if top_10_asks.len() < 11 { break; }
                    top_10_asks.pop_last();
                }
                println!("Top 10 bids \n {:?}", top_10_bids);
                println!("Top 10 asks \n {:?}", top_10_asks);
            }
            (exch, Err(e)) => {
                continue;
            }
        }
    }
    Ok(())
}