use crate::{
    http::{Query, Request},
    schema::MetaResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/intraday/meta)
///
/// Fetching today's basic informations.
///
pub struct MetaRequest<'a> {
    odd_lot: bool,
    symbol_id: &'a str,
}

impl Default for MetaRequest<'_> {
    fn default() -> Self {
        MetaRequest::new()
    }
}

impl<'a> MetaRequest<'a> {
    pub fn new() -> Self {
        MetaRequest {
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

impl Request for MetaRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/realtime/v0.3/intraday/meta";
    type Response = MetaResponse;

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
