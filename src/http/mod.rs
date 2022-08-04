pub mod intraday;
pub use intraday::IntradayBuilder;

pub mod marketdata;
pub use marketdata::MarketdataBuilder;

use serde::de::DeserializeOwned;
use std::time::Duration;

#[cfg(feature = "query")]
use ureq::{Agent, AgentBuilder, OrAnyStatus};

#[cfg(feature = "async-query")]
use reqwest::{Client, ClientBuilder, RequestBuilder as AsyncReq};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::Result,
};

#[derive(Default)]
pub struct RequestBuilder<'a> {
    token: &'a str,
    read_timeout_sec: u64,
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> RequestBuilder<'a> {
        RequestBuilder {
            token: "demo",
            read_timeout_sec: 3,
        }
    }

    pub fn token(mut self, token: &'a str) -> RequestBuilder {
        self.token = token;
        self
    }

    pub fn read_timeout_sec(mut self, sec: u64) -> RequestBuilder<'a> {
        self.read_timeout_sec = sec;
        self
    }

    pub fn build(&self) -> Result<BlockRequest<'a>> {
        Ok(BlockRequest {
            token: self.token,
            agent: AgentBuilder::new()
                .timeout_read(Duration::from_secs(self.read_timeout_sec))
                .build(),
        })
    }

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
