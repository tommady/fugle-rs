use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::schema::{default_date_time, Info};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dealt {
    #[serde(default = "default_date_time")]
    pub at: DateTime<FixedOffset>,
    #[serde(default)]
    pub bid: f64,
    #[serde(default)]
    pub ask: f64,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub volume: u64,
    #[serde(default)]
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
#[serde(rename_all = "camelCase")]
pub struct DealtsData {
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub dealts: Vec<Dealt>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtsResponse {
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub data: DealtsData,
}
