use std::env;

use crypto_stream::connect_web_stream;

#[tokio::main]
async fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a symbol and port to subscribe to for example ethbtc 50001");
        return;
    }
    let symbol = String::from(&args[1]);
    let port: i32 = args[2].parse().unwrap();

    connect_web_stream(&symbol, port).await.expect("Could not connect to websocket api");
}
