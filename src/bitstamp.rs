use std::error::Error;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt};
use tokio::net::TcpStream;
use url;
pub async fn connect_bitstamp(symbol: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
    let bitstamp_path = String::from("wss://ws.bitstamp.net");
    let bitstamp_url = url::Url::parse(&bitstamp_path).expect("Could not connect to coinbase");
    let (mut bitstamp_ws_stream, _) = connect_async(bitstamp_url).await?;
    let bitstamp_subscribe_json = format!("{{\"event\": \"bts:subscribe\", \"data\": {{  \"channel\": \"order_book_{symbol}\"}} }}");
    bitstamp_ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(bitstamp_subscribe_json)).await?;

    Ok(bitstamp_ws_stream)
}