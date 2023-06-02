use serde::Deserialize;
use serde_this_or_that::as_f64;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Quotes {
    #[serde(deserialize_with = "as_f64")]
    pub(crate) price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub(crate) amount: f64,
}

pub struct TopQuotes {
    pub(crate) bids: [Quotes; 10],
    pub(crate) asks: [Quotes; 10],
}

#[derive(Debug, Copy, Clone)]
pub enum Exchanges {
    Bitstamp = 0,
    Binance = 1,
    None = 2,
}

#[derive(Debug, Copy, Clone)]
pub struct Level {
    pub(crate) exchange: Exchanges,
    pub(crate) price: f64,
    pub(crate) amount: f64,
}


