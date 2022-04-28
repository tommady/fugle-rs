pub mod chart;
pub mod dealts;
pub mod meta;
pub mod quote;
pub mod volumes;

use std::time::Duration;

use ureq::{Agent, AgentBuilder};

use crate::intraday::{
    chart::ChartBuilder, dealts::DealtsBuilder, meta::MetaBuilder, quote::QuoteBuilder,
    volumes::VolumesBuilder,
};

const INTRADAY_CHART: &str = "https://api.fugle.tw/realtime/v0.3/intraday/chart";
const INTRADAY_QUOTE: &str = "https://api.fugle.tw/realtime/v0.3/intraday/quote";
const INTRADAY_META: &str = "https://api.fugle.tw/realtime/v0.3/intraday/meta";
const INTRADAY_DEALTS: &str = "https://api.fugle.tw/realtime/v0.3/intraday/dealts";
const INTRADAY_VOLUMES: &str = "https://api.fugle.tw/realtime/v0.3/intraday/volumes";

/// Accumulates options towards building an Intraday instance.
pub struct IntradayBuilder<'a> {
    token: &'a str,
    agent_builder: AgentBuilder,
}

impl<'a> Default for IntradayBuilder<'a> {
    fn default() -> IntradayBuilder<'static> {
        IntradayBuilder::new()
    }
}

impl<'a> IntradayBuilder<'a> {
    /// Returns a default IntradayBuilder with
    /// * fugle "demo" token
    /// * [ default ureq agent settings ] ( https://github.com/algesten/ureq/blob/main/src/agent.rs#L202 )
    pub fn new() -> IntradayBuilder<'a> {
        IntradayBuilder {
            token: "demo",
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
    /// # use fugle::intraday::IntradayBuilder;
    /// let agent = IntradayBuilder::new()
    ///     .token("b52153ae36747b17c8bdee801da19542")
    ///     .build();
    /// ```
    pub fn token(mut self, token: &'a str) -> IntradayBuilder {
        self.token = token;
        self
    }

    /// Setup http read timeout option.
    ///
    /// By default there is no read timeout.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::intraday::IntradayBuilder;
    /// let agent = IntradayBuilder::new()
    ///     .read_timeout_sec(10) // set read timeout in 10 seconds
    ///     .build();
    /// ```
    pub fn read_timeout_sec(mut self, sec: u64) -> IntradayBuilder<'a> {
        self.agent_builder = self.agent_builder.timeout_read(Duration::from_secs(sec));
        self
    }

    /// Create a new Intraday instance.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::intraday::IntradayBuilder;
    /// let agent = IntradayBuilder::new().build();
    /// ```
    pub fn build(self) -> Intraday<'a> {
        Intraday {
            token: self.token,
            agent: self.agent_builder.build(),
        }
    }
}

/// Intraday is the RESTful API queryer to request fugle endpoints.
pub struct Intraday<'a> {
    token: &'a str,
    agent: Agent,
}

impl<'a> Intraday<'a> {
    /// [Endpoint](https://developer.fugle.tw/document/intraday/chart)
    ///
    /// Fetching the current drawing data.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.chart("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn chart(&self, symbol_id: &str) -> ChartBuilder {
        ChartBuilder {
            request: self
                .agent
                .get(INTRADAY_CHART)
                .query("apiToken", self.token)
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
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.quote("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote(&self, symbol_id: &str) -> QuoteBuilder {
        QuoteBuilder {
            request: self
                .agent
                .get(INTRADAY_QUOTE)
                .query("apiToken", self.token)
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
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.meta("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn meta(&self, symbol_id: &str) -> MetaBuilder {
        MetaBuilder {
            request: self
                .agent
                .get(INTRADAY_META)
                .query("apiToken", self.token)
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
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn dealts(&self, symbol_id: &str) -> DealtsBuilder {
        DealtsBuilder {
            request: self
                .agent
                .get(INTRADAY_DEALTS)
                .query("apiToken", self.token)
                .query("symbolId", symbol_id),
        }
    }

    /// [Endpoint](https://developer.fugle.tw/document/intraday/volumes)
    ///
    /// Fetching today's volume information.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.volumes("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn volumes(&self, symbol_id: &str) -> VolumesBuilder {
        VolumesBuilder {
            request: self
                .agent
                .get(INTRADAY_VOLUMES)
                .query("apiToken", self.token)
                .query("symbolId", symbol_id),
        }
    }
}
