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
    pub(crate) bids: [Quotes; 20],
    pub(crate) asks: [Quotes; 20],
}

pub struct TopQuotesWithExchange{
    pub(crate) exchange: Exchanges,
    pub(crate) quotes: TopQuotes,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Exchanges {
    Bitstamp = 0,
    Binance = 1,
}

pub fn exchange_to_str(exch: Exchanges) -> String {
    match exch {
        Exchanges::Binance => String::from("Binance"),
        Exchanges::Bitstamp => String::from("Bitstamp")
    }
}


