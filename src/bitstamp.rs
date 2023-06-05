use std::error::Error;

use futures_util::SinkExt;
use serde::Deserialize;
use serde_this_or_that::as_u64;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use url;

use crate::common::*;

#[derive(Debug, Deserialize)]
struct Data {
    #[serde(deserialize_with = "as_u64")]
    timestamp: u64,
    #[serde(deserialize_with = "as_u64")]
    microtimestamp: u64,
    bids: Vec<Quotes>,
    asks: Vec<Quotes>,
}

#[derive(Debug, Deserialize)]
struct BitstampMsg {
    data: Data,
    channel: String,
    event: String,
}

pub async fn connect(symbol: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
    let bitstamp_path = String::from("wss://ws.bitstamp.net");
    let bitstamp_url = url::Url::parse(&bitstamp_path).expect("Could not connect to coinbase");
    let (mut bitstamp_ws_stream, _) = connect_async(bitstamp_url).await?;
    let bitstamp_subscribe_json = format!("{{\"event\": \"bts:subscribe\", \"data\": {{  \"channel\": \"order_book_{symbol}\"}} }}");
    bitstamp_ws_stream.send(Message::Text(bitstamp_subscribe_json)).await?;

    Ok(bitstamp_ws_stream)
}

pub fn decode(msg: &Message) -> Result<TopQuotesWithExchange, Box<dyn Error>> {

    let bitstamp_msg: BitstampMsg = serde_json::from_str(&msg.to_string())?;
    let quotes: TopQuotes = TopQuotes {
        bids: <[Quotes; 20]>::try_from(&bitstamp_msg.data.bids[0..20])?,
        asks: <[Quotes; 20]>::try_from(&bitstamp_msg.data.asks[0..20])?,
    };

    Ok(TopQuotesWithExchange{exchange: Exchanges::Bitstamp,quotes})
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_bitstamp() {
        let json_input = std::fs::read_to_string("test/bitstamp_input.txt").expect("Could not read file");
        let bitstamp_msg: serde_json::Result<BitstampMsg> = serde_json::from_str(&json_input);
        let bitstamp_msg = bitstamp_msg.expect("Could not deserialize json");
        assert!((bitstamp_msg.data.bids[0].amount - 0.79318434).abs() < 1e-9);
    }
}