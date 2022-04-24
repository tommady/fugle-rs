use serde::{Deserialize, Serialize};

use crate::schema::Info;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Volume {
    pub price: f64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeData {
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub volumes: Vec<Volume>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumesResponse {
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub data: VolumeData,
}
