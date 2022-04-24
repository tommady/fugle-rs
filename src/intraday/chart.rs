use serde::{Deserialize, Serialize};

use crate::schema::Info;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Chart {
    #[serde(rename = "o")]
    pub open: Vec<f64>,
    #[serde(rename = "h")]
    pub high: Vec<f64>,
    #[serde(rename = "l")]
    pub low: Vec<f64>,
    #[serde(rename = "c")]
    pub close: Vec<f64>,
    #[serde(rename = "v")]
    pub volume: Vec<u64>,
    #[serde(rename = "t")]
    pub unix_timestamp: Vec<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub chart: Chart,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartResponse {
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub data: ChartData,
}
