use crate::schema::{ErrorResponse, Response, Result};
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Request};

const INTRADAY_CHART: &str = "https://api.fugle.tw/realtime/v0.2/intraday/chart";
const INTRADAY_QUOTE: &str = "https://api.fugle.tw/realtime/v0.2/intraday/quote";
const INTRADAY_META: &str = "https://api.fugle.tw/realtime/v0.2/intraday/meta";
const INTRADAY_DEALTS: &str = "https://api.fugle.tw/realtime/v0.2/intraday/dealts";

pub struct IntradayBuilder {
    token: &'static str,
    agent_builder: AgentBuilder,
}

impl IntradayBuilder {
    pub fn new() -> IntradayBuilder {
        IntradayBuilder {
            token: "demo",
            agent_builder: AgentBuilder::new(),
        }
    }

    pub fn token(mut self, token: &'static str) -> IntradayBuilder {
        self.token = token;
        self
    }

    pub fn read_timeout_sec(mut self, sec: u64) -> IntradayBuilder {
        self.agent_builder = self.agent_builder.timeout_read(Duration::from_secs(sec));
        self
    }

    pub fn build(self) -> Intraday {
        Intraday {
            token: self.token,
            agent: self.agent_builder.build(),
        }
    }
}

pub struct Intraday {
    token: &'static str,
    agent: Agent,
}

impl Intraday {
    /// [Endpoint]: https://developer.fugle.tw/document/intraday/chart
    ///
    /// Fetching the current drawing data.
    pub fn chart(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            request: self
                .agent
                .get(INTRADAY_CHART)
                .query("apiToken", self.token)
                .query("symboldId", symbol_id),
        }
    }

    /// [Endpoint]: https://developer.fugle.tw/document/intraday/quote
    ///
    /// Fetching the current status and statistics.
    pub fn quote(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            request: self
                .agent
                .get(INTRADAY_QUOTE)
                .query("apiToken", self.token)
                .query("symboldId", symbol_id),
        }
    }

    /// [Endpoint]: https://developer.fugle.tw/document/intraday/meta
    ///
    /// Fetching today's basic informations.
    pub fn meta(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            request: self
                .agent
                .get(INTRADAY_META)
                .query("apiToken", self.token)
                .query("symboldId", symbol_id),
        }
    }

    /// [Endpoint]: https://developer.fugle.tw/document/intraday/dealts
    ///
    /// Fetching today's advantage information.
    pub fn dealts(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            request: self
                .agent
                .get(INTRADAY_DEALTS)
                .query("apiToken", self.token)
                .query("symboldId", symbol_id),
        }
    }
}

pub struct GetQueryBuilder {
    request: Request,
}

impl GetQueryBuilder {
    pub fn limit(mut self, limit: usize) -> GetQueryBuilder {
        self.request = self.request.query("limit", &limit.to_string());
        self
    }

    pub fn offset(mut self, offset: usize) -> GetQueryBuilder {
        self.request = self.request.query("offset", &offset.to_string());
        self
    }

    pub fn odd_lot(mut self, odd_lot: bool) -> GetQueryBuilder {
        self.request = self.request.query("oddLot", &odd_lot.to_string());
        self
    }

    pub fn call(self) -> Result<Response> {
        let response = self.request.call()?;
        if response.status() != 200 {
            let err: ErrorResponse = response.into_json()?;
            return Err(err.into());
        }
        Ok(response.into_json()?)
    }
}
