use serde::{Deserialize, Serialize};
use ureq::{OrAnyStatus, Request};

use crate::{
    errors::{ErrorResponse, FugleError},
    schema::{Info, Result},
};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Volume {
    pub price: f64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct VolumeData {
    pub info: Info,
    pub volumes: Vec<Volume>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct VolumesResponse {
    pub api_version: String,
    pub data: VolumeData,
}

/// Associate options when doing the request.
pub struct VolumesBuilder {
    pub request: Request,
}

impl VolumesBuilder {
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
    /// agent.volumes("2884")
    /// .odd_lot(true)
    /// .call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn odd_lot(mut self, odd_lot: bool) -> VolumesBuilder {
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
    /// agent.volumes("2884").call()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn call(self) -> Result<VolumesResponse> {
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
        let it = VolumesBuilder {
            request: AgentBuilder::new().build().get("not-exists-endpoint"),
        };
        assert!(it.call().is_err());
    }
}
