use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use url;
#[tokio::main]
async fn main()  {
    let binance_path = String::from("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms");
    let url = url::Url::parse(&path).expect("Could not parse url");
    let (ws_stream, _) = connect_async(url).await.expect("Could not connect to websocket");
    let (_,mut read) = ws_stream.split();
    loop {
        let msg = read.next().await.unwrap();
        println!("{:?}", msg);
    }
}
