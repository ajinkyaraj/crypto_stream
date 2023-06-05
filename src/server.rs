use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use tokio::sync::RwLock;
use tonic::Status;

use orderbook::orderbook_aggregator_server::OrderbookAggregator;

use crate::SummaryChannel;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}


#[derive(Debug)]
pub struct OrderbookService {
    summary: Arc<RwLock<SummaryChannel>>,
}

impl OrderbookService {
    pub fn new(summary: Arc<RwLock<SummaryChannel>>) -> Self {
        Self { summary }
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookService {
    type BookSummaryStream =
    Pin<Box<dyn Stream<Item=Result<orderbook::Summary, Status>> + Send + 'static>>;
    async fn book_summary(
        &self,
        request: tonic::Request<orderbook::Empty>,
    ) -> Result<tonic::Response<Self::BookSummaryStream>, Status> {
        let _req = request.into_inner();

        let mut summary_rx = self.summary.read().await.1.clone();

        let output = async_stream::try_stream! {
            // yield the current value
            let result_rx = summary_rx.borrow().clone();
            yield result_rx;
            while (summary_rx.changed().await).is_ok() {
               let result_rx = summary_rx.borrow().clone();
                yield result_rx;
            }
        };

        Ok(tonic::Response::new(Box::pin(output) as Self::BookSummaryStream))
    }
}