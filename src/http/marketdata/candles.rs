use crate::{
    http::{Query, Request},
    schema::CandlesResponse,
};

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

// use ureq::{OrAnyStatus, Request};
//
// use crate::{
//     errors::{ErrorResponse, FugleError},
//     schema::{CandlesResponse, Result},
// };
//
// /// Associate options when doing the request.
// pub struct CandlesBuilder {
//     pub request: Request,
// }
//
// impl CandlesBuilder {
//     /// The start day of the history stock information.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::MarketdataBuilder;
//     ///
//     /// let agent = MarketdataBuilder::new().build();
//     ///
//     /// agent.candles("2884").from("2022-02-07").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn from(mut self, day: &str) -> CandlesBuilder {
//         self.request = self.request.query("from", day);
//         self
//     }
//
//     /// The end day of the history stock information.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::MarketdataBuilder;
//     ///
//     /// let agent = MarketdataBuilder::new().build();
//     ///
//     /// agent.candles("2884")
//     /// .from("2022-02-07")
//     /// .to("2022-02-11")
//     /// .call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn to(mut self, day: &str) -> CandlesBuilder {
//         self.request = self.request.query("to", day);
//         self
//     }
//
//     /// Send the request.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::MarketdataBuilder;
//     ///
//     /// let agent = MarketdataBuilder::new().build();
//     ///
//     /// agent.candles("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn call(self) -> Result<CandlesResponse> {
//         match self.request.call().or_any_status() {
//             Ok(response) => {
//                 if response.status() > 200 {
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
//         let it = CandlesBuilder {
//             request: AgentBuilder::new().build().get("not-exists-endpoint"),
//         };
//         assert!(it.call().is_err());
//     }
// }
