use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{ChartResponse, Result},
};

/// Associate options when doing the request.
pub struct ChartBuilder {
    pub request: Request,
}

impl ChartBuilder {
    /// To see odd lotter or not.
    /// Default value on fugle API is false
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::http::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.chart("2884")
    /// .odd_lot(true)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn odd_lot(mut self, odd_lot: bool) -> ChartBuilder {
        self.request = self.request.query("oddLot", &odd_lot.to_string());
        self
    }

    /// Send the request.
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::http::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    ///
    /// agent.chart("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn call(self) -> Result<ChartResponse> {
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
        let it = ChartBuilder {
            request: AgentBuilder::new().build().get("not-exists-endpoint"),
        };
        assert!(it.call().is_err());
    }
}
