use crate::{
    http::{Query, Request},
    schema::VolumesResponse,
};

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

// /// Associate options when doing the request.
// pub struct VolumesBuilder {
//     pub request: Request,
// }
//
// impl VolumesBuilder {
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
//     /// agent.volumes("2884")
//     /// .odd_lot(true)
//     /// .call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn odd_lot(mut self, odd_lot: bool) -> VolumesBuilder {
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
//     /// agent.volumes("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn call(self) -> Result<VolumesResponse> {
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
//         let it = VolumesBuilder {
//             request: AgentBuilder::new().build().get("not-exists-endpoint"),
//         };
//         assert!(it.call().is_err());
//     }
// }