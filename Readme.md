# Crypto_Stream
THis library provides code for connecting to crypto exchange's websocket api
s and receive stream for a provided crypto symbol pair (for ex. ethbtc)

## Usage 
~~~
cargo run --bin server ethbtc 50001

<open another terminal>
cargo run --bin client 50001
~~~

## Libraries/Data structures Chosen
1. Websocket: To connect to websocket API and receive the orderbook stream continuosly from the exchange, we used tokio-tungstenite library. 
   1. tokio-tungstenite library is well tested and reliably used in various web frameworks (ex. axum) and applications. Although it is not the fastest websocket library but it is easiest to use and its integration with tokio async runtime is excellent. It also offers sufficient performance for our task. 
   2. We are connecting to two exchanges which provide orderbook data at interval of 100ms and 200 ms (binance and bitstamp respectively.) 
   3. The size of data received in bytes is 1192 bytes and 5741 bytes at frequency of 100 ms and 200 ms respectively. 
   4. Combining the two the total throughput is 11920 + 28705 bytes/sec = 40625 Bps = 325 Kbps. 
   5. According to the benchmark at https://github.com/PrivateRookie/ws-tool the tungstenite library can process 5.1912 Gbps of load. ( 300 bytes of msg, 2268088.00 msgs/second)
   6. So we can add 100 more exchanges providing order depth and still tokio tungestenite would be sufficient. 
   7. But if we want even further optimization in performance we would use fastwebsockets ( check benchmark at https://github.com/nurmohammed840/web-socket-benchmark/tree/main
   8. Fastwebsocket documentation is very low level websocket api where we have to handle most of the connection stuff and handshake. 
2. After receiving the order depth from two exchanges:
   1. Clip the top 20 bid and ask in the order book received from the websocket msg.
   2. store the top 20 bid and ask in hashmap with exchange as the key.
   3. after every update for any exchange, merge the bids and asks vectors of each exchange to find best 10 bid and ask. The assumption is that each vector is already in sorted order as it was received from exchange.
   4. The best bid is the one with highest price and if prices are equal then on with highest amount. The best ask is the one with lowest price (or highest amount if price is equal).
   5.
3. Sending the synthesized order book via gRPC
   1. The gRPC library used is tonic. tonic is batteries included high performance gRPC framework.
   2. Its very easy to use with great documentation.
   3. For a single core server, rust-tonic is top performer according to following benchmarks
      1. https://github.com/LesnyRumcajs/grpc_bench/wiki/2022-04-23-bench-results
   4. we spawn a new task for gRPC server and send the summary data structure in protocol buffer format.
4. gRPC client
   1. A cli based progress bar interface which shows top 10 bids and asks.
   2. Taken from indicatif crate.