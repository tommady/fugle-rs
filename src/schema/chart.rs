use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::Info;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Chart {
    #[serde(rename = "o")]
    pub open: Vec<Decimal>,
    #[serde(rename = "h")]
    pub high: Vec<Decimal>,
    #[serde(rename = "l")]
    pub low: Vec<Decimal>,
    #[serde(rename = "c")]
    pub close: Vec<Decimal>,
    #[serde(rename = "v")]
    pub volume: Vec<u64>,
    #[serde(rename = "t")]
    pub unix_timestamp: Vec<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ChartData {
    pub info: Info,
    pub chart: Chart,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ChartResponse {
    pub api_version: String,
    pub data: ChartData,
}
