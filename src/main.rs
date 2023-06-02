use std::env;
use crypto_stream::connect_web_stream;
#[tokio::main]
async fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a symbol to subscribe to for example ethbtc");
        return;
    }
    let symbol = String::from(&args[1]);
    connect_web_stream(&symbol).await.expect("Could not connect to websocket api");
}
