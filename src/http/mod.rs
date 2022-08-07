pub mod intraday;

pub mod marketdata;
pub use marketdata::MarketdataBuilder;

use serde::de::DeserializeOwned;
use std::time::Duration;

#[cfg(feature = "query")]
use ureq::{Agent, AgentBuilder, OrAnyStatus};

#[cfg(feature = "async-query")]
use reqwest::{Client, ClientBuilder};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::Result,
};

/// Accumulates options towards building an Restful api instance.
pub struct RestfulBuilder<'a> {
    token: &'a str,
    read_timeout_sec: u64,
}

impl<'a> Default for RestfulBuilder<'a> {
    fn default() -> RestfulBuilder<'a> {
        RestfulBuilder::new()
    }
}

impl<'a> RestfulBuilder<'a> {
    /// Returns a default RestfulBuilder with
    /// * fugle "demo" token
    /// * 3 seconds read timeout
    pub fn new() -> RestfulBuilder<'a> {
        RestfulBuilder {
            token: "demo",
            read_timeout_sec: 3,
        }
    }

    /// Setup your personal fugle token.
    ///
    /// By default the RestfulBuilder using fugle demo token which has limitations on querying,
    /// please login into below web page
    /// https://developer.fugle.tw/
    /// then find your personal token.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::http::RestfulBuilder ;
    /// let agent = RestfulBuilder ::new()
    ///     .token("b52153ae36747b17c8bdee801da19542")
    ///     .build();
    /// ```
    pub fn token(mut self, token: &'a str) -> RestfulBuilder {
        self.token = token;
        self
    }

    /// Setup http read timeout option.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::http::RestfulBuilder ;
    /// let agent = RestfulBuilder ::new()
    ///     .read_timeout_sec(10) // set read timeout in 10 seconds
    ///     .build();
    /// ```
    pub fn read_timeout_sec(mut self, sec: u64) -> RestfulBuilder<'a> {
        self.read_timeout_sec = sec;
        self
    }

    /// Create a new Block Request instance.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::http::RestfulBuilder ;
    /// let agent = RestfulBuilder ::new().build();
    /// ```
    #[cfg(feature = "query")]
    pub fn build(&self) -> Result<BlockRequest<'a>> {
        Ok(BlockRequest {
            token: self.token,
            agent: AgentBuilder::new()
                .timeout_read(Duration::from_secs(self.read_timeout_sec))
                .build(),
        })
    }

    /// Create a new Aync Request instance.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::http::RestfulBuilder ;
    /// let agent = RestfulBuilder ::new().build_async();
    /// ```
    pub fn build_async(&self) -> Result<AsyncRequest<'a>> {
        Ok(AsyncRequest {
            token: self.token,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(self.read_timeout_sec))
                .build()?,
        })
    }
}

pub struct Query {
    pub param: String,
    pub value: String,
}

pub trait Request {
    const REQUEST_URL: &'static str;
    type Response: DeserializeOwned;

    fn queries(&self) -> Vec<Query>;
}

pub struct BlockRequest<'a> {
    token: &'a str,
    agent: Agent,
}

impl<'a> BlockRequest<'a> {
    pub fn call<R>(&self, request: R) -> Result<R::Response>
    where
        R: Request,
    {
        let mut req = self.agent.get(R::REQUEST_URL).query("apiToken", self.token);

        for r in request.queries() {
            req = req.query(&r.param, &r.value)
        }

        match req.call().or_any_status() {
            Ok(res) => {
                if res.status() != 200 {
                    let err: ErrorResponse = res.into_json()?;
                    return Err(err.into());
                }
                Ok(res.into_json()?)
            }
            Err(e) => Err(FugleError::Ureq(Box::new(e.into()))),
        }
    }
}

pub struct AsyncRequest<'a> {
    token: &'a str,
    client: Client,
}

impl<'a> AsyncRequest<'a> {
    pub async fn call<R>(&self, request: R) -> Result<R::Response>
    where
        R: Request,
    {
        let mut req = self
            .client
            .get(R::REQUEST_URL)
            .query(&[("apiToken", self.token)]);

        for r in request.queries() {
            req = req.query(&[(&r.param, &r.value)])
        }

        match req.send().await {
            Ok(res) => {
                if res.status() != 200 {
                    let err: ErrorResponse = res.json().await?;
                    return Err(err.into());
                }
                Ok(res.json().await?)
            }
            Err(e) => Err(FugleError::Reqwest(e)),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::http::intraday::chart::ChartRequest;
//     use crate::http::intraday::dealts::DealtsRequest;
//
//     #[test]
//     fn test_block_request() {
//         let cres = RequestBuilder::new()
//             .build()
//             .unwrap()
//             .call(ChartRequest {
//                 odd_lot: false,
//                 symbol_id: "2884",
//             })
//             .unwrap();
//         println!("chart : {:?}", cres);
//
//         let mut d = DealtsRequest::default();
//         d.symbol_id("2884");
//         let dres = RequestBuilder::new().build().unwrap().call(d).unwrap();
//         println!("dealts : {:?}", dres);
//     }
//
//     #[tokio::test]
//     async fn test_async_request() {
//         let cres = RequestBuilder::new()
//             .build_async()
//             .unwrap()
//             .call(ChartRequest {
//                 odd_lot: false,
//                 symbol_id: "2884",
//             })
//             .await
//             .unwrap();
//         println!("chart : {:?}", cres);
//
//         let mut d = DealtsRequest::default();
//         d.symbol_id("2884");
//         let dres = RequestBuilder::new()
//             .build_async()
//             .unwrap()
//             .call(d)
//             .await
//             .unwrap();
//         println!("dealts : {:?}", dres);
//     }
// }
