use crate::{
    http::{Query, Request},
    schema::CandlesResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/marketdata/candles)
///
/// Fetching history stock information.
///
pub struct CandlesRequest<'a> {
    from: &'a str,
    to: &'a str,
    symbol_id: &'a str,
}

impl Default for CandlesRequest<'_> {
    fn default() -> Self {
        CandlesRequest::new()
    }
}

impl<'a> CandlesRequest<'a> {
    pub fn new() -> Self {
        CandlesRequest {
            symbol_id: "2884",
            from: "",
            to: "",
        }
    }

    pub fn symbol_id(mut self, symbol_id: &'a str) -> Self {
        self.symbol_id = symbol_id;
        self
    }

    pub fn from(mut self, day: &'a str) -> Self {
        self.from = day;
        self
    }

    pub fn to(mut self, day: &'a str) -> Self {
        self.to = day;
        self
    }
}

impl Request for CandlesRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/marketdata/v0.3/candles";
    type Response = CandlesResponse;

    fn queries(&self) -> Vec<Query> {
        vec![
            Query {
                param: "symbolId".to_string(),
                value: self.symbol_id.to_string(),
            },
            Query {
                param: "from".to_string(),
                value: self.from.to_string(),
            },
            Query {
                param: "to".to_string(),
                value: self.to.to_string(),
            },
        ]
    }
}
