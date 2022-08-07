use crate::{
    http::{Query, Request},
    schema::MetaResponse,
};

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

// /// Associate options when doing the request.
// pub struct MetaBuilder {
//     pub request: Request,
// }
//
// impl MetaBuilder {
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
//     /// agent.meta("2884")
//     /// .odd_lot(true)
//     /// .call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn odd_lot(mut self, odd_lot: bool) -> MetaBuilder {
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
//     /// agent.meta("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn call(self) -> Result<MetaResponse> {
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
//         let it = MetaBuilder {
//             request: AgentBuilder::new().build().get("not-exists-endpoint"),
//         };
//         assert!(it.call().is_err());
//     }
// }
