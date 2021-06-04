use crate::schema::{ErrorResponse, FugleError, Response, ResponseType, Result};
use std::time::Duration;
use ureq::{Agent, AgentBuilder, OrAnyStatus, Request};

const INTRADAY_CHART: &str = "https://api.fugle.tw/realtime/v0.2/intraday/chart";
const INTRADAY_QUOTE: &str = "https://api.fugle.tw/realtime/v0.2/intraday/quote";
const INTRADAY_META: &str = "https://api.fugle.tw/realtime/v0.2/intraday/meta";
const INTRADAY_DEALTS: &str = "https://api.fugle.tw/realtime/v0.2/intraday/dealts";

/// Accumulates options towards building an Intraday instance.
pub struct IntradayBuilder {
    token: String,
    agent_builder: AgentBuilder,
}

impl Default for IntradayBuilder {
    fn default() -> IntradayBuilder {
        IntradayBuilder::new()
    }
}

impl IntradayBuilder {
    /// Returns a default IntradayBuilder with
    /// * fugle "demo" token
    /// * [ default ureq agent settings ] ( https://github.com/algesten/ureq/blob/main/src/agent.rs#L202 )
    pub fn new() -> IntradayBuilder {
        IntradayBuilder {
            token: "demo".to_owned(),
            agent_builder: AgentBuilder::new(),
        }
    }

    /// Setup your personal fugle token.
    ///
    /// By default the IntradayBuilder using fugle demo token which has limitations on querying,
    /// please login into below web page
    /// https://developer.fugle.tw/
    /// then find your personal token.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::crawler::IntradayBuilder;
    /// let agent = IntradayBuilder::new()
    ///     .token("b52153ae36747b17c8bdee801da19542")
    ///     .build();
    /// ```
    pub fn token(mut self, token: &str) -> IntradayBuilder {
        self.token = token.to_owned();
        self
    }

    /// Setup http read timeout option.
    ///
    /// By default there is no read timeout.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::crawler::IntradayBuilder;
    /// let agent = IntradayBuilder::new()
    ///     .read_timeout_sec(10) // set read timeout in 10 seconds
    ///     .build();
    /// ```
    pub fn read_timeout_sec(mut self, sec: u64) -> IntradayBuilder {
        self.agent_builder = self.agent_builder.timeout_read(Duration::from_secs(sec));
        self
    }

    /// Create a new Intraday instance.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::crawler::IntradayBuilder;
    /// let agent = IntradayBuilder::new().build();
    /// ```
    pub fn build(self) -> Intraday {
        Intraday {
            token: self.token,
            agent: self.agent_builder.build(),
        }
    }
}

/// Intraday is the RESTful API queryer to request fugle endpoints.
pub struct Intraday {
    token: String,
    agent: Agent,
}

impl Intraday {
    /// [Endpoint](https://developer.fugle.tw/document/intraday/chart)
    ///
    /// Fetching the current drawing data.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.chart("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn chart(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            resposne_type: ResponseType::Chart,
            request: self
                .agent
                .get(INTRADAY_CHART)
                .query("apiToken", &self.token)
                .query("symbolId", symbol_id),
        }
    }

    /// [Endpoint](https://developer.fugle.tw/document/intraday/quote)
    ///
    /// Fetching the current status and statistics.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.quote("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            resposne_type: ResponseType::Quote,
            request: self
                .agent
                .get(INTRADAY_QUOTE)
                .query("apiToken", &self.token)
                .query("symbolId", symbol_id),
        }
    }

    /// [Endpoint](https://developer.fugle.tw/document/intraday/meta)
    ///
    /// Fetching today's basic informations.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.meta("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn meta(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            resposne_type: ResponseType::Meta,
            request: self
                .agent
                .get(INTRADAY_META)
                .query("apiToken", &self.token)
                .query("symbolId", symbol_id),
        }
    }

    /// [Endpoint](https://developer.fugle.tw/document/intraday/dealts)
    ///
    /// Fetching today's advantage information.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn dealts(&self, symbol_id: &str) -> GetQueryBuilder {
        GetQueryBuilder {
            resposne_type: ResponseType::Dealts,
            request: self
                .agent
                .get(INTRADAY_DEALTS)
                .query("apiToken", &self.token)
                .query("symbolId", symbol_id),
        }
    }
}

/// Associate options when doing the request.
pub struct GetQueryBuilder {
    request: Request,
    resposne_type: ResponseType,
}

impl GetQueryBuilder {
    /// Set a limit param while using dealts request.
    /// Default value on fugle API is 0
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884")
    /// .limit(99)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn limit(mut self, limit: usize) -> GetQueryBuilder {
        self.request = self.request.query("limit", &limit.to_string());
        self
    }

    /// Set an offset param while using dealts request.
    /// Default value on fugle API is 50
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884")
    /// .offset(3)
    /// .limit(6)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn offset(mut self, offset: usize) -> GetQueryBuilder {
        self.request = self.request.query("offset", &offset.to_string());
        self
    }

    /// To see odd lotter or not.
    /// Default value on fugle API is false
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.meta("2884")
    /// .odd_lot(true)
    /// .call()?;
    ///
    /// agent.quote("2884")
    /// .odd_lot(true)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn odd_lot(mut self, odd_lot: bool) -> GetQueryBuilder {
        self.request = self.request.query("oddLot", &odd_lot.to_string());
        self
    }

    /// Send the request.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::crawler::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.meta("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn call(self) -> Result<Response> {
        match self.request.call().or_any_status() {
            Ok(response) => {
                if response.status() != 200 {
                    let err: ErrorResponse = response.into_json()?;
                    return Err(err.into());
                }
                match self.resposne_type {
                    ResponseType::Chart => return Ok(Response::Chart(response.into_json()?)),
                    ResponseType::Meta => return Ok(Response::Meta(response.into_json()?)),
                    ResponseType::Quote => return Ok(Response::Quote(response.into_json()?)),
                    ResponseType::Dealts => return Ok(Response::Dealts(response.into_json()?)),
                }
            }
            Err(e) => return Err(FugleError::Ureq(Box::new(e.into()))),
        };
        // if response.status() != 200 {
        //     let err: ErrorResponse = response.into_json()?;
        //     return Err(err.into());
        // }

        // match self.resposne_type {
        //     ResponseType::Chart => Ok(Response::Chart(response.into_json()?)),
        //     ResponseType::Meta => Ok(Response::Meta(response.into_json()?)),
        //     ResponseType::Quote => Ok(Response::Quote(response.into_json()?)),
        //     ResponseType::Dealts => Ok(Response::Dealts(response.into_json()?)),
        // }
    }
}

// ```
// use ureq::Error::Status;
// # fn main() -> std::result::Result<(), ureq::Transport> {
// # ureq::is_test(true);
// use ureq::OrAnyStatus;
//
// let resp = ureq::get("http://example.com/")
//   .call()
//   .or_any_status()?;
// # Ok(())
// # }
// ```
