use serde::Deserialize;
use serde_this_or_that::as_f64;
#[derive(Debug, Deserialize)]
pub struct Quotes{
    #[serde(deserialize_with = "as_f64")]
    pub (crate) price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub(crate) amount : f64
}


