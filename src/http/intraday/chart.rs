use crate::{
    http::{Query, Request},
    schema::ChartResponse,
};

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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::http::RestfulBuilder;
//
//     #[test]
//     fn test_chart_request_blocking() {
//         let client = RestfulBuilder::new().build().unwrap();
//         let req = ChartRequest::new().odd_lot(true).symbol_id("2884");
//         assert!(client.call(req).is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_chart_request_async() {
//         let client = RestfulBuilder::new().build_async().unwrap();
//         let req = ChartRequest::new().odd_lot(true).symbol_id("2884");
//         assert!(client.call(req).await.is_ok());
//     }
// }

// /// Associate options when doing the request.
// pub struct ChartBuilder {
//     pub request: Request,
// }
//
// impl ChartBuilder {
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
//     /// agent.chart("2884")
//     /// .odd_lot(true)
//     /// .call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn odd_lot(mut self, odd_lot: bool) -> ChartBuilder {
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
//     /// agent.chart("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn call(self) -> Result<ChartResponse> {
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
//         let it = ChartBuilder {
//             request: AgentBuilder::new().build().get("not-exists-endpoint"),
//         };
//         assert!(it.call().is_err());
//     }
// }
