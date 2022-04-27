use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{default_date_time, Info, Result},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Dealt {
    pub at: DateTime<FixedOffset>,
    pub bid: f64,
    pub ask: f64,
    pub price: f64,
    pub volume: u64,
    pub serial: u64,
}

impl Default for Dealt {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> Dealt {
        Dealt {
            at: default_date_time(),
            bid: 0.0,
            ask: 0.0,
            price: 0.0,
            volume: 0,
            serial: 0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DealtsData {
    pub info: Info,
    pub dealts: Vec<Dealt>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DealtsResponse {
    pub api_version: String,
    pub data: DealtsData,
}

/// Associate options when doing the request.
pub struct IntradayDealtsBuilder {
    pub request: Request,
}

impl IntradayDealtsBuilder {
    /// Set a limit param while using dealts request.
    /// Default value on fugle API is 0
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884")
    /// .limit(99)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn limit(mut self, limit: usize) -> IntradayDealtsBuilder {
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
    /// # use fugle::intraday::IntradayBuilder;
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
    pub fn offset(mut self, offset: usize) -> IntradayDealtsBuilder {
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
    /// # use fugle::intraday::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.dealts("2884")
    /// .odd_lot(true)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn odd_lot(mut self, odd_lot: bool) -> IntradayDealtsBuilder {
        self.request = self.request.query("oddLot", &odd_lot.to_string());
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
    /// agent.dealts("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn call(self) -> Result<DealtsResponse> {
        match self.request.call().or_any_status() {
            Ok(response) => {
                if response.status() != 200 {
                    let err: ErrorResponse = response.into_json()?;
                    return Err(err.into());
                }
                Ok(response.into_json()?)
            }
            Err(e) => Err(FugleError::Ureq(Box::new(e.into()))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ureq::AgentBuilder;

    #[test]
    fn test_call_failed_on_transport() {
        let it = IntradayDealtsBuilder {
            request: AgentBuilder::new().build().get("not-exists-endpoint"),
        };
        assert!(it.call().is_err());
    }
}
