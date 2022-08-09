use crate::{
    http::{Query, Request},
    schema::dealts::DealtsResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/intraday/dealts)
///
/// Fetching today's advantage information.
///
pub struct DealtsRequest<'a> {
    symbol_id: &'a str,
    odd_lot: bool,
    limit: usize,
    offset: usize,
}

impl Default for DealtsRequest<'_> {
    fn default() -> Self {
        DealtsRequest::new()
    }
}

impl<'a> DealtsRequest<'a> {
    pub fn new() -> Self {
        DealtsRequest {
            symbol_id: "2884",
            odd_lot: false,
            limit: 0,
            offset: 0,
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

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}

impl Request for DealtsRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/realtime/v0.3/intraday/dealts";
    type Response = DealtsResponse;

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
            Query {
                param: "limit".to_string(),
                value: self.limit.to_string(),
            },
            Query {
                param: "offset".to_string(),
                value: self.offset.to_string(),
            },
        ]
    }
}
