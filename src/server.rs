use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary, Level};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}