use std::error::Error;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use url;

use serde::Deserialize;

use crate::common::{Exchanges, Quotes, TopQuotes, TopQuotesWithExchange};

pub async fn connect(symbol: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
    let binance_path = format!("wss://stream.binance.com:9443/ws/{symbol}@depth20@100ms");
    let binance_url = url::Url::parse(&binance_path).expect("Could not parse url");
    let (binance_ws_stream, _) = connect_async(binance_url).await?;
    Ok(binance_ws_stream)
}


#[derive(Debug, Deserialize)]
struct BinanceMsg {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<Quotes>,
    asks: Vec<Quotes>,
}

pub fn decode(msg: &Message) -> Result<TopQuotesWithExchange, Box<dyn Error>> {

    println!("msg size is : {}", msg.len());
    let binance_msg: BinanceMsg = serde_json::from_str(&msg.to_string())?;
    let quotes: TopQuotes = TopQuotes {
        bids: <[Quotes; 20]>::try_from(&binance_msg.bids[0..20])?,
        asks: <[Quotes; 20]>::try_from(&binance_msg.asks[0..20])?,
    };

    Ok(TopQuotesWithExchange{exchange: Exchanges::Binance, quotes})
}

// Add a test which supplies binance_input.txt as input file and decodes json output using BinanceMsg struct

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_binance() {
        let json_input = std::fs::read_to_string("test/binance_input.txt").expect("Could not read file");
        let binance_msg: serde_json::Result<BinanceMsg> = serde_json::from_str(&json_input);
        let binance_msg = binance_msg.expect("Could not deserialize json");
        assert!((binance_msg.bids[0].amount - 69.98270000).abs() < 1e-9);
    }
}
