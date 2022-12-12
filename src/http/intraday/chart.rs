use crate::{
    http::{Query, Request},
    schema::ChartResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/intraday/chart)
///
/// Fetching the current drawing data.
///
pub struct ChartRequest<'a> {
    odd_lot: bool,
    symbol_id: &'a str,
}

impl Default for ChartRequest<'_> {
    fn default() -> Self {
        ChartRequest::new()
    }
}

impl<'a> ChartRequest<'a> {
    pub fn new() -> Self {
        ChartRequest {
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

impl Request for ChartRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/realtime/v0.3/intraday/chart";
    type Response = ChartResponse;

    fn queries(&self) -> Vec<Query> {
        let mut ret = Vec::with_capacity(2);
        if !self.symbol_id.is_empty() {
            ret.push(Query {
                param: "symbolId".to_string(),
                value: self.symbol_id.to_string(),
            })
        }
        if self.odd_lot {
            ret.push(Query {
                param: "oddLot".to_string(),
                value: self.odd_lot.to_string(),
            })
        }
        ret
    }
}
