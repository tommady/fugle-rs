use chrono::{naive::MIN_DATE, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, FugleError>;

// // for listener usage, but fugle websocket not provide DealtsResponse currently.
// // for more detail please check at
// // https://developer.fugle.tw/document/status
// #[derive(Debug)]
// pub enum WssResponse {
//     ChartResponse(ChartResponse),
//     MetaResponse(MetaResponse),
//     QuoteResponse(QuoteResponse),
//     // DealtsResponse,
// }

fn default_naive_date() -> NaiveDate {
    MIN_DATE
}

fn default_naive_date_time() -> NaiveDateTime {
    MIN_DATE.and_hms(0, 0, 0)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(default = "default_naive_date_time")]
    pub last_updated_at: NaiveDateTime,
    #[serde(default = "default_naive_date")]
    pub date: NaiveDate,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub symbol_id: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub time_zone: String,
}

impl Default for Info {
    fn default() -> Info {
        Info {
            last_updated_at: MIN_DATE.and_hms(0, 0, 0),
            date: MIN_DATE,
            ..std::default::Default::default()
        }
    }
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
    pub chart: HashMap<NaiveDateTime, Chart>,
}

// #[derive(Default, Debug, Deserialize, Serialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct ChartResponse {
//     pub api_version: String,
//     pub data: ChartData,
// }

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

// #[derive(Default, Debug, Deserialize, Serialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct MetaResponse {
//     pub api_version: String,
//     pub data: MetaData,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTotal {
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
    #[serde(default)]
    pub unit: u64,
    #[serde(default)]
    pub volume: u64,
}

impl Default for QuoteTotal {
    fn default() -> QuoteTotal {
        QuoteTotal {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrial {
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub unit: u64,
    #[serde(default)]
    pub volume: u64,
}

impl Default for QuoteTrial {
    fn default() -> QuoteTrial {
        QuoteTrial {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrade {
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub unit: u64,
    #[serde(default)]
    pub volume: u64,
    #[serde(default)]
    pub serial: u64,
}

impl Default for QuoteTrade {
    fn default() -> QuoteTrade {
        QuoteTrade {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteBest {
    pub price: f64,
    pub unit: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOrder {
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
    #[serde(default)]
    pub best_bids: Vec<QuoteBest>,
    #[serde(default)]
    pub best_asks: Vec<QuoteBest>,
}

impl Default for QuoteOrder {
    fn default() -> QuoteOrder {
        QuoteOrder {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceHigh {
    #[serde(default)]
    pub price: f64,
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
}

impl Default for QuotePriceHigh {
    fn default() -> QuotePriceHigh {
        QuotePriceHigh {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceLow {
    #[serde(default)]
    pub price: f64,
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
}

impl Default for QuotePriceLow {
    fn default() -> QuotePriceLow {
        QuotePriceLow {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePriceOpen {
    #[serde(default)]
    pub price: f64,
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
}

impl Default for QuotePriceOpen {
    fn default() -> QuotePriceOpen {
        QuotePriceOpen {
            at: MIN_DATE.and_hms(0, 0, 0),
            ..std::default::Default::default()
        }
    }
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
    pub order: QuoteOrder,
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

// #[derive(Default, Debug, Deserialize, Serialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct QuoteResponse {
//     pub api_version: String,
//     pub data: QuoteData,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dealt {
    #[serde(default = "default_naive_date_time")]
    pub at: NaiveDateTime,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub unit: u64,
    #[serde(default)]
    pub serial: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DealtsData {
    pub info: Info,
    pub dealts: Vec<Dealt>,
}

// #[derive(Default, Debug, Deserialize, Serialize)]
// #[serde(default, rename_all = "camelCase")]
// pub struct DealtsResponse {
//     pub api_version: String,
//     pub data: DealtsData,
// }
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
enum Data {
    DealtsData(DealtsData),
    QuoteData(QuoteData),
    MetaData(MetaData),
    ChartData(ChartData),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub api_version: String,
    pub data: Data,
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
    // error from ureq lib
    Ureq(ureq::Error),
    // error from std io
    StdIO(std::io::Error),
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
            FugleError::Ureq(ref e) => write!(f, "Ureq Lib error: {}", e),
            FugleError::StdIO(ref e) => write!(f, "std io json Deserialize error: {}", e),
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
            FugleError::Ureq(ref e) => Some(e),
            FugleError::StdIO(ref e) => Some(e),
            FugleError::General(ref _e) => None,
            FugleError::Unknown(ref _e) => None,
            FugleError::Unauthorized => None,
            FugleError::RateLimitExceeded => None,
            FugleError::ResourceNotFound => None,
            FugleError::MpscSendError => None,
        }
    }
}

impl From<std::io::Error> for FugleError {
    fn from(err: std::io::Error) -> FugleError {
        FugleError::StdIO(err)
    }
}

impl From<ureq::Error> for FugleError {
    fn from(err: ureq::Error) -> FugleError {
        FugleError::Ureq(err)
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
