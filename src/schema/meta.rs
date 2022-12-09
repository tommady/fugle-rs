use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::Info;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Meta {
    pub market: String,
    pub name_zh_tw: String,
    pub industry_zh_tw: String,
    pub price_reference: Decimal,
    pub price_high_limit: Decimal,
    pub price_low_limit: Decimal,
    pub can_day_buy_sell: bool,
    pub can_day_sell_buy: bool,
    pub can_short_margin: bool,
    pub can_short_lend: bool,
    pub trading_unit: u64,
    pub currency: String,
    pub is_terminated: bool,
    pub is_suspended: bool,
    pub type_zh_tw: String,
    pub abnormal: String,
    pub is_unusually_recommended: bool,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub meta: Meta,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaResponse {
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub data: MetaData,
}
