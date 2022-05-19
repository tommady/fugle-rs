use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{CandlesResponse, Result},
};

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
    /// # use fugle::http::MarketdataBuilder;
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
    /// # use fugle::http::MarketdataBuilder;
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
    /// # use fugle::http::MarketdataBuilder;
    ///
    /// let agent = MarketdataBuilder::new().build();
    ///
    /// agent.candles("2884").call()?;
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

#[cfg(test)]
mod test {
    use super::*;
    use ureq::AgentBuilder;

    #[test]
    fn test_call_failed_on_transport() {
        let it = CandlesBuilder {
            request: AgentBuilder::new().build().get("not-exists-endpoint"),
        };
        assert!(it.call().is_err());
    }
}
