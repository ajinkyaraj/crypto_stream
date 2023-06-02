use tokio_tungstenite::{connect_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use std::env;
use url;

#[tokio::main]
async fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a symbol to subscribe to for example ethbtc");
        return;
    }
    let symbol = String::from("ethbtc");
    let binance_path = String::from("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms");
    let bitstamp_path : String = String::from("wss://ws.bitstamp.net");

    let binance_url = url::Url::parse(&binance_path).expect("Could not parse url");
    let bitstamp_url = url::Url::parse(&bitstamp_path).expect("Could not connect to coinbase");
    // use tokio-tungstenite to connect to binance websocket api in asynchronous manner.
    // Refer to examples/client.rs example in tokio-tungstenite github repo for more details.
    let (binance_ws_stream, _) = connect_async(binance_url).await.expect("Could not connect to binance websocket api");
    let (bitstamp_ws_stream, _) = connect_async(bitstamp_url).await.expect("Could not connect to bitstamp websocket api");
    let (_,mut binance_read) = binance_ws_stream.split();
    let (mut bitstamp_write,mut bitstamp_read) = bitstamp_ws_stream.split();
    // subscribe to symbol for bitstamp as per https://www.bitstamp.net/websocket/v2/
    let bitstamp_subscribe_json = format!("{{\"event\": \"bts:subscribe\", \"data\": {{  \"channel\": \"order_book_{symbol}\"}} }}");
    bitstamp_write.send(tokio_tungstenite::tungstenite::Message::Text(bitstamp_subscribe_json)).await.expect("Could not subscribe to bitstamp websocket api");

     loop {

         let binance_msg =  binance_read.next();
         let bitstamp_msg =  bitstamp_read.next();
         let msg = tokio::select! {
             Some(msg) = binance_msg => {println!("Binance Msg:"); msg},
             Some(msg) = bitstamp_msg => { println!("Bitstamp msg:"); msg},
             else => break,
         };
         println!("{:?}", msg);
     }
}
