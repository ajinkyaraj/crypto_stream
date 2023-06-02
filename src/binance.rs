use std::error::Error;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

use url;

 pub async fn connect_binance(symbol: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
     let binance_path = format!("wss://stream.binance.com:9443/ws/{symbol}@depth20@100ms");
     let binance_url = url::Url::parse(&binance_path).expect("Could not parse url");
     let (binance_ws_stream, _) = connect_async(binance_url).await?;
     Ok(binance_ws_stream)
 }
