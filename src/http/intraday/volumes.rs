use crate::{
    http::{Query, Request},
    schema::VolumesResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/intraday/volumes)
///
/// Fetching today's volume information.
///
pub struct VolumesRequest<'a> {
    odd_lot: bool,
    symbol_id: &'a str,
}

impl Default for VolumesRequest<'_> {
    fn default() -> Self {
        VolumesRequest::new()
    }
}

impl<'a> VolumesRequest<'a> {
    pub fn new() -> Self {
        VolumesRequest {
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

impl Request for VolumesRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/realtime/v0.3/intraday/volumes";
    type Response = VolumesResponse;

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
