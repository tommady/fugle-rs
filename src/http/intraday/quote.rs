use crate::{
    http::{Query, Request},
    schema::QuoteResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/intraday/quote)
///
/// Fetching the current status and statistics.
///
pub struct QuoteRequest<'a> {
    odd_lot: bool,
    symbol_id: &'a str,
}

impl Default for QuoteRequest<'_> {
    fn default() -> Self {
        QuoteRequest::new()
    }
}

impl<'a> QuoteRequest<'a> {
    pub fn new() -> Self {
        QuoteRequest {
            symbol_id: "2884",
            odd_lot: false,
        }
    }

    pub fn symbol_id(mut self, symbol_id: &'a str) -> Self {
        self.symbol_id = symbol_id;
        self
    }

    pub fn odd_lot(mut self, odd_lot: bool) -> Self {
        self.odd_lot = odd_lot;
        self
    }
}

impl Request for QuoteRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/realtime/v0.3/intraday/quote";
    type Response = QuoteResponse;

    fn queries(&self) -> Vec<Query> {
        vec![
            Query {
                param: "symbolId".to_string(),
                value: self.symbol_id.to_string(),
            },
            Query {
                param: "oddLot".to_string(),
                value: self.odd_lot.to_string(),
            },
        ]
    }
}
