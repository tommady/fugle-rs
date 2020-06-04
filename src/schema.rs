use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// for listener usage, but fugle websocket not provide DealtsResponse currently.
#[derive(Debug)]
pub enum Response {
    ChartResponse(ChartResponse),
    MetaResponse(MetaResponse),
    QuoteResponse(QuoteResponse),
    // DealtsResponse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub last_updated_at: Option<DateTime<Utc>>,
    pub date: String,
    pub mode: String,
    pub symbol_id: String,
    pub country_code: String,
    pub time_zone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub info: Info,
    pub chart: HashMap<DateTime<Utc>, Chart>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartResponse {
    pub api_version: String,
    pub data: ChartData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub is_index: bool,
    pub name_zh_tw: String,
    pub industry_zh_tw: String,
    pub price_reference: f64,
    pub price_high_limit: f64,
    pub price_low_limit: f64,
    pub can_day_buy_sell: bool,
    pub can_day_sell_buy: bool,
    pub can_short_margin: bool,
    pub can_short_lend: bool,
    pub volume_per_unit: u64,
    pub currency: String,
    pub is_terminated: bool,
    pub is_suspended: bool,
    pub is_warrant: bool,
    pub type_zh_tw: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub info: Info,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaResponse {
    pub api_version: String,
    pub data: MetaData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTotal {
    pub at: DateTime<Utc>,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrial {
    pub at: DateTime<Utc>,
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrade {
    pub at: DateTime<Utc>,
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
    pub serial: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteBest {
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOrder {
    pub at: DateTime<Utc>,
    pub best_bids: Vec<QuoteBest>,
    pub best_asks: Vec<QuoteBest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceHigh {
    pub price: f64,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceLow {
    pub price: f64,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceOpen {
    pub price: f64,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub is_curbing: bool,
    pub is_trial: bool,
    pub is_open_delayed: bool,
    pub is_close_delayed: bool,
    pub is_halting: bool,
    pub is_closed: bool,
    pub total: QuoteTotal,
    pub trial: QuoteTrial,
    pub trade: QuoteTrade,
    pub order: Option<QuoteOrder>,
    pub price_high: QuotePriceHigh,
    pub price_low: QuotePriceLow,
    pub price_open: QuotePriceOpen,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    pub info: Info,
    pub quote: Quote,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub api_version: String,
    pub data: QuoteData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dealt {
    pub at: DateTime<Utc>,
    pub price: f64,
    pub unit: u64,
    pub serial: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtsData {
    pub info: Info,
    pub dealts: Vec<Dealt>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealtsResponse {
    pub api_version: String,
    pub data: DealtsData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub api_version: String,
    pub error: Error,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FugleError: {{ api_version:{}, code:{}, msg:{} }}",
            self.api_version, self.error.code, self.error.message,
        )
    }
}

impl error::Error for ErrorResponse {
    fn description(&self) -> &str {
        "API response an error"
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
