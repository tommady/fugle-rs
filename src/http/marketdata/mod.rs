mod candles;
pub use candles::CandlesRequest;

// use std::time::Duration;
//
// use ureq::{Agent, AgentBuilder};
//
// use crate::http::marketdata::candles::CandlesBuilder;
//
// const MARKETDATA_CANDLES: &str = "https://api.fugle.tw/marketdata/v0.3/candles";
//
// /// Accumulates options towards building an Marketdata instance.
// pub struct MarketdataBuilder<'a> {
//     token: &'a str,
//     agent_builder: AgentBuilder,
// }
//
// impl<'a> Default for MarketdataBuilder<'a> {
//     fn default() -> MarketdataBuilder<'static> {
//         MarketdataBuilder::new()
//     }
// }
//
// impl<'a> MarketdataBuilder<'a> {
//     /// Returns a default MarketdataBuilder with
//     /// * fugle "demo" token
//     /// * [ default ureq agent settings ] ( https://github.com/algesten/ureq/blob/main/src/agent.rs#L202 )
//     pub fn new() -> MarketdataBuilder<'a> {
//         MarketdataBuilder {
//             token: "demo",
//             agent_builder: AgentBuilder::new(),
//         }
//     }
//
//     /// Setup your personal fugle token.
//     ///
//     /// By default the MarketdataBuilder using fugle demo token which has limitations on querying,
//     /// please login into below web page
//     /// https://developer.fugle.tw/
//     /// then find your personal token.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # use fugle::http::MarketdataBuilder;
//     /// let agent = MarketdataBuilder::new()
//     ///     .token("b52153ae36747b17c8bdee801da19542")
//     ///     .build();
//     /// ```
//     pub fn token(mut self, token: &'a str) -> MarketdataBuilder {
//         self.token = token;
//         self
//     }
//
//     /// Setup http read timeout option.
//     ///
//     /// By default there is no read timeout.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # use fugle::http::MarketdataBuilder;
//     /// let agent = MarketdataBuilder::new()
//     ///     .read_timeout_sec(10) // set read timeout in 10 seconds
//     ///     .build();
//     /// ```
//     pub fn read_timeout_sec(mut self, sec: u64) -> MarketdataBuilder<'a> {
//         self.agent_builder = self.agent_builder.timeout_read(Duration::from_secs(sec));
//         self
//     }
//
//     /// Create a new Marketdata instance.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # use fugle::http::MarketdataBuilder;
//     /// let agent = MarketdataBuilder::new().build();
//     /// ```
//     pub fn build(self) -> Marketdata<'a> {
//         Marketdata {
//             token: self.token,
//             agent: self.agent_builder.build(),
//         }
//     }
// }
//
// /// Marketdata is the RESTful API queryer to request fugle endpoints.
// pub struct Marketdata<'a> {
//     token: &'a str,
//     agent: Agent,
// }
//
// impl<'a> Marketdata<'a> {
//     /// [Endpoint](https://developer.fugle.tw/docs/data/marketdata/candles)
//     ///
//     /// Fetching history stock information.
//     ///
//     /// # Example:
//     ///
//     /// ```
//     /// # fn main() -> fugle::schema::Result<()> {
//     /// # use fugle::http::MarketdataBuilder;
//     ///
//     /// let agent = MarketdataBuilder::new().build();
//     /// agent.candles("2884").call()?;
//     ///
//     /// # Ok(())
//     /// # }
//     /// ```
//     pub fn candles(&self, symbol_id: &str) -> CandlesBuilder {
//         CandlesBuilder {
//             request: self
//                 .agent
//                 .get(MARKETDATA_CANDLES)
//                 .query("apiToken", self.token)
//                 .query("symbolId", symbol_id),
//         }
//     }
// }
