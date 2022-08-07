use crate::{
    http::{Query, Request},
    schema::QuoteResponse,
};

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

// /// Associate options when doing the request.
// pub struct QuoteBuilder {
//     pub request: Request,
// }
//
// impl QuoteBuilder {
//     /// To see odd lotter or not.
//     /// Default value on fugle API is false
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::IntradayBuilder;
//     ///
//     /// let agent = IntradayBuilder::new().build();
//     ///
//     /// agent.quote("2884")
//     /// .odd_lot(true)
//     /// .call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn odd_lot(mut self, odd_lot: bool) -> QuoteBuilder {
//         self.request = self.request.query("oddLot", &odd_lot.to_string());
//         self
//     }
//
//     /// Send the request.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::IntradayBuilder;
//     ///
//     /// let agent = IntradayBuilder::new().build();
//     ///
//     /// agent.quote("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn call(self) -> Result<QuoteResponse> {
//         match self.request.call().or_any_status() {
//             Ok(response) => {
//                 if response.status() != 200 {
//                     let err: ErrorResponse = response.into_json()?;
//                     return Err(err.into());
//                 }
//                 Ok(response.into_json()?)
//             }
//             Err(e) => Err(FugleError::Ureq(Box::new(e.into()))),
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     use ureq::AgentBuilder;
//
//     #[test]
//     fn test_call_failed_on_transport() {
//         let it = QuoteBuilder {
//             request: AgentBuilder::new().build().get("not-exists-endpoint"),
//         };
//         assert!(it.call().is_err());
//     }
// }
