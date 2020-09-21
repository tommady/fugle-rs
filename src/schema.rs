use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, FugleError>;

// for listener usage, but fugle websocket not provide DealtsResponse currently.
#[derive(Debug)]
pub enum Response {
    ChartResponse(ChartResponse),
    MetaResponse(MetaResponse),
    QuoteResponse(QuoteResponse),
    // DealtsResponse,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Info {
    pub last_updated_at: Option<DateTime<Utc>>,
    pub date: String,
    pub mode: String,
    pub symbol_id: String,
    pub country_code: String,
    pub time_zone: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Chart {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ChartData {
    pub info: Info,
    pub chart: HashMap<Option<DateTime<Utc>>, Chart>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ChartResponse {
    pub api_version: String,
    pub data: ChartData,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
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

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MetaData {
    pub info: Info,
    pub meta: Meta,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MetaResponse {
    pub api_version: String,
    pub data: MetaData,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteTotal {
    pub at: Option<DateTime<Utc>>,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteTrial {
    pub at: Option<DateTime<Utc>>,
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteTrade {
    pub at: Option<DateTime<Utc>>,
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
    pub serial: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteBest {
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteOrder {
    pub at: Option<DateTime<Utc>>,
    pub best_bids: Vec<QuoteBest>,
    pub best_asks: Vec<QuoteBest>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuotePriceHigh {
    pub price: f64,
    pub at: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuotePriceLow {
    pub price: f64,
    pub at: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuotePriceOpen {
    pub price: f64,
    pub at: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
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

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteData {
    pub info: Info,
    pub quote: Quote,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteResponse {
    pub api_version: String,
    pub data: QuoteData,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Dealt {
    pub at: Option<DateTime<Utc>>,
    pub price: f64,
    pub unit: u64,
    pub serial: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DealtsData {
    pub info: Info,
    pub dealts: Vec<Dealt>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DealtsResponse {
    pub api_version: String,
    pub data: DealtsData,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ErrorResponse {
    pub api_version: String,
    pub error: Error,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FugleAPI: {{ api_version:{}, code:{}, msg:{} }}",
            self.api_version, self.error.code, self.error.message,
        )
    }
}

impl std::error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum FugleError {
    MpscSendError,
    // error from serde_json lib
    SerdeJson(serde_json::Error),
    // error from tungstenite lib
    Tungstenite(tungstenite::Error),
    // error from reqwest lib
    Reqwest(reqwest::Error),
    // Url Parsing error
    Url(url::ParseError),
    // from fugle API response code, to specific errors
    // https://developer.fugle.tw/document/intraday/introduction
    // 400
    General(ErrorResponse),
    // 401
    Unauthorized,
    // 403
    RateLimitExceeded,
    // 404
    ResourceNotFound,
    // status codes not in the list
    Unknown(ErrorResponse),
}

impl std::fmt::Display for FugleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FugleError::SerdeJson(ref e) => write!(f, "Serde_json Lib error: {}", e),
            FugleError::Tungstenite(ref e) => write!(f, "Tungstenite Lib error: {}", e),
            FugleError::Reqwest(ref e) => write!(f, "Reqwest Lib error: {}", e),
            FugleError::Url(ref e) => write!(f, "Url Parse error: {}", e),
            FugleError::General(ref e) => write!(f, "General purpose error: {}", e),
            FugleError::Unknown(ref e) => write!(f, "Unknown error: {}", e),
            FugleError::Unauthorized => write!(f, "Unauthorized"),
            FugleError::RateLimitExceeded => write!(f, "Rate limit or quota exceeded"),
            FugleError::ResourceNotFound => write!(f, "Resource Not Found"),
            FugleError::MpscSendError => write!(f, "MPSC Send Error"),
        }
    }
}

impl std::error::Error for FugleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FugleError::SerdeJson(ref e) => Some(e),
            FugleError::Tungstenite(ref e) => Some(e),
            FugleError::Reqwest(ref e) => Some(e),
            FugleError::Url(ref e) => Some(e),
            FugleError::General(ref _e) => None,
            FugleError::Unknown(ref _e) => None,
            FugleError::Unauthorized => None,
            FugleError::RateLimitExceeded => None,
            FugleError::ResourceNotFound => None,
            FugleError::MpscSendError => None,
        }
    }
}

impl From<url::ParseError> for FugleError {
    fn from(err: url::ParseError) -> FugleError {
        FugleError::Url(err)
    }
}

impl From<reqwest::Error> for FugleError {
    fn from(err: reqwest::Error) -> FugleError {
        FugleError::Reqwest(err)
    }
}

impl From<tungstenite::Error> for FugleError {
    fn from(err: tungstenite::Error) -> FugleError {
        FugleError::Tungstenite(err)
    }
}

impl From<serde_json::Error> for FugleError {
    fn from(err: serde_json::Error) -> FugleError {
        FugleError::SerdeJson(err)
    }
}

impl From<ErrorResponse> for FugleError {
    fn from(err: ErrorResponse) -> FugleError {
        match err.error.code {
            400 => FugleError::General(err),
            401 => FugleError::Unauthorized,
            403 => FugleError::RateLimitExceeded,
            404 => FugleError::ResourceNotFound,
            _ => FugleError::Unknown(err),
        }
    }
}
