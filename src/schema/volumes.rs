use serde::{Deserialize, Serialize};

use crate::schema::Info;

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
