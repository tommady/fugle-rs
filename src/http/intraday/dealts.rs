use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{dealts::DealtsResponse, Result},
};

/// Associate options when doing the request.
pub struct DealtsBuilder {
    pub request: Request,
}

impl DealtsBuilder {
    /// Set a limit param while using dealts request.
    /// Default value on fugle API is 0
    ///
    /// # Example:
    ///
    /// ```
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::http::IntradayBuilder;
    ///
    /// let agent = IntradayBuilder::new().build();
    /// agent.dealts("2884")
    /// .limit(99)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn limit(mut self, limit: usize) -> DealtsBuilder {
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
    /// # use fugle::http::IntradayBuilder;
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
    pub fn offset(mut self, offset: usize) -> DealtsBuilder {
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
    /// # use fugle::http::IntradayBuilder;
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
    pub fn odd_lot(mut self, odd_lot: bool) -> DealtsBuilder {
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
        let it = DealtsBuilder {
            request: AgentBuilder::new().build().get("not-exists-endpoint"),
        };
        assert!(it.call().is_err());
    }
}
