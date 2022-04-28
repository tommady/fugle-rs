use serde::{Deserialize, Serialize};
use time::Date;
use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{de_date, Result},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Candle {
    #[serde(deserialize_with = "de_date")]
    pub date: Date,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

impl Default for Candle {
    fn default() -> Candle {
        Candle {
            date: Date::MIN,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CandlesResponse {
    pub symbol_id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub exchange: String,
    pub market: String,
    pub candles: Vec<Candle>,
}

impl Default for CandlesResponse {
    fn default() -> CandlesResponse {
        CandlesResponse {
            symbol_id: "".to_string(),
            typ: "".to_string(),
            exchange: "".to_string(),
            market: "".to_string(),
            candles: vec![],
        }
    }
}

/// Associate options when doing the request.
pub struct CandlesBuilder {
    pub request: Request,
}

impl CandlesBuilder {
    /// The start day of the history stock information.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::marketdata::MarketdataBuilder;
    ///
    /// let agent = MarketdataBuilder::new().build();
    ///
    /// agent.candles("2884").from("2022-02-07").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from(mut self, day: &str) -> CandlesBuilder {
        self.request = self.request.query("from", day);
        self
    }

    /// The end day of the history stock information.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::marketdata::MarketdataBuilder;
    ///
    /// let agent = MarketdataBuilder::new().build();
    ///
    /// agent.candles("2884")
    /// .from("2022-02-07")
    /// .to("2022-02-11")
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn to(mut self, day: &str) -> CandlesBuilder {
        self.request = self.request.query("to", day);
        self
    }

    /// Send the request.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.volumes("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn call(self) -> Result<CandlesResponse> {
        match self.request.call().or_any_status() {
            Ok(response) => {
                if response.status() > 200 {
                    let err: ErrorResponse = response.into_json()?;
                    return Err(err.into());
                }
                Ok(response.into_json()?)
            }
            Err(e) => Err(FugleError::Ureq(Box::new(e.into()))),
        }
    }
}
