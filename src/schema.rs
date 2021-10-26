#![cfg_attr(coverage, feature(no_coverage))]
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, FugleError>;

#[cfg_attr(coverage, no_coverage)]
fn default_naive_date() -> NaiveDate {
    NaiveDate::from_num_days_from_ce(1)
}

#[cfg_attr(coverage, no_coverage)]
fn default_date_time() -> DateTime<FixedOffset> {
    DateTime::<FixedOffset>::from_utc(
        NaiveDateTime::from_timestamp(0, 0),
        FixedOffset::east(8 * 3600),
    )
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(default = "default_date_time")]
    pub last_updated_at: DateTime<FixedOffset>,
    #[serde(default = "default_naive_date")]
    pub date: NaiveDate,
    #[serde(default)]
    pub symbol_id: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub time_zone: String,
    #[serde(default)]
    pub exchange: String,
    #[serde(default)]
    pub market: String,
    #[serde(default, rename = "type")]
    pub typ: String,
}

impl Default for Info {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> Info {
        Info {
            last_updated_at: default_date_time(),
            date: default_naive_date(),
            symbol_id: "".to_string(),
            country_code: "".to_string(),
            time_zone: "".to_string(),
            exchange: "".to_string(),
            market: "".to_string(),
            typ: "".to_string(),
        }
    }
}

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

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Meta {
    pub market: String,
    pub name_zh_tw: String,
    pub industry_zh_tw: String,
    pub price_reference: f64,
    pub price_high_limit: f64,
    pub price_low_limit: f64,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTotal {
    #[serde(default = "default_date_time")]
    pub at: DateTime<FixedOffset>,
    #[serde(default)]
    pub transaction: u64,
    #[serde(default)]
    pub trade_value: f64,
    #[serde(default)]
    pub trade_volume: u64,
    #[serde(default)]
    pub trade_volume_at_bid: u64,
    #[serde(default)]
    pub trade_volume_at_ask: u64,
    #[serde(default)]
    pub bid_orders: u64,
    #[serde(default)]
    pub ask_orders: u64,
    #[serde(default)]
    pub bid_volume: u64,
    #[serde(default)]
    pub ask_volume: u64,
    #[serde(default)]
    pub serial: u64,
}

impl Default for QuoteTotal {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> QuoteTotal {
        QuoteTotal {
            at: default_date_time(),
            transaction: 0,
            trade_value: 0.0,
            trade_volume: 0,
            trade_volume_at_bid: 0,
            trade_volume_at_ask: 0,
            bid_orders: 0,
            ask_orders: 0,
            bid_volume: 0,
            ask_volume: 0,
            serial: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrial {
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
}

impl Default for QuoteTrial {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> QuoteTrial {
        QuoteTrial {
            at: default_date_time(),
            bid: 0.0,
            ask: 0.0,
            price: 0.0,
            volume: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTrade {
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

impl Default for QuoteTrade {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> QuoteTrade {
        QuoteTrade {
            at: default_date_time(),
            price: 0.0,
            bid: 0.0,
            ask: 0.0,
            volume: 0,
            serial: 0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct QuoteBidAsk {
    pub price: f64,
    pub volume: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOrder {
    #[serde(default = "default_date_time")]
    pub at: DateTime<FixedOffset>,
    #[serde(default)]
    pub bids: Vec<QuoteBidAsk>,
    #[serde(default)]
    pub asks: Vec<QuoteBidAsk>,
}

impl Default for QuoteOrder {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> QuoteOrder {
        QuoteOrder {
            at: default_date_time(),
            bids: Vec::with_capacity(0),
            asks: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePrice {
    #[serde(default)]
    pub price: f64,
    #[serde(default = "default_date_time")]
    pub at: DateTime<FixedOffset>,
}

impl Default for QuotePrice {
    #[cfg_attr(coverage, no_coverage)]
    fn default() -> QuotePrice {
        QuotePrice {
            at: default_date_time(),
            price: 0.0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    #[serde(default)]
    pub is_curbing: bool,
    #[serde(default)]
    pub is_curbing_rise: bool,
    #[serde(default)]
    pub is_curbing_fall: bool,
    #[serde(default)]
    pub is_trial: bool,
    #[serde(default)]
    pub is_open_delayed: bool,
    #[serde(default)]
    pub is_close_delayed: bool,
    #[serde(default)]
    pub is_halting: bool,
    #[serde(default)]
    pub is_closed: bool,
    #[serde(default)]
    pub total: QuoteTotal,
    #[serde(default)]
    pub trial: QuoteTrial,
    #[serde(default)]
    pub trade: QuoteTrade,
    #[serde(default)]
    pub order: QuoteOrder,
    #[serde(default)]
    pub price_high: QuotePrice,
    #[serde(default)]
    pub price_low: QuotePrice,
    #[serde(default)]
    pub price_open: QuotePrice,
    #[serde(default)]
    pub price_avg: QuotePrice,
    #[serde(default)]
    pub change: f64,
    #[serde(default)]
    pub change_percent: f64,
    #[serde(default)]
    pub amplitude: f64,
    #[serde(default)]
    pub price_limit: u8,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub quote: Quote,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub data: QuoteData,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    Chart(ChartResponse),
    Quote(Box<QuoteResponse>),
    Meta(MetaResponse),
    Dealts(DealtsResponse),
    Volumes(VolumesResponse),
}

#[cfg_attr(coverage, no_coverage)]
#[derive(Debug)]
pub enum ResponseType {
    Chart,
    Quote,
    Meta,
    Dealts,
    Volumes,
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
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FugleAPI: {{ api_version:{}, code:{}, msg:{} }}",
            self.api_version, self.error.code, self.error.message,
        )
    }
}

impl std::error::Error for ErrorResponse {
    #[cfg_attr(coverage, no_coverage)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg_attr(coverage, no_coverage)]
#[derive(Debug)]
pub enum FugleError {
    MpscSendError,
    MpscRecvError(std::sync::mpsc::RecvError),
    // error from serde_json lib
    SerdeJson(serde_json::Error),
    // error from tungstenite lib
    #[cfg(feature = "websocket")]
    Tungstenite(tungstenite::Error),
    // error from ureq lib
    Ureq(Box<ureq::Error>),
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
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FugleError::SerdeJson(ref e) => write!(f, "Serde_json Lib error: {}", e),
            #[cfg(feature = "websocket")]
            FugleError::Tungstenite(ref e) => write!(f, "Tungstenite Lib error: {}", e),
            FugleError::Ureq(ref e) => write!(f, "Ureq Lib error: {}", e),
            FugleError::StdIO(ref e) => write!(f, "std io json Deserialize error: {}", e),
            FugleError::General(ref e) => write!(f, "General purpose error: {}", e),
            FugleError::Unknown(ref e) => write!(f, "Unknown error: {}", e),
            FugleError::Unauthorized => write!(f, "Unauthorized"),
            FugleError::RateLimitExceeded => write!(f, "Rate limit or quota exceeded"),
            FugleError::ResourceNotFound => write!(f, "Resource Not Found"),
            FugleError::MpscSendError => write!(f, "MPSC Send Error"),
            FugleError::MpscRecvError(ref e) => write!(f, "MPSC Receive Error: {}", e),
        }
    }
}

impl std::error::Error for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FugleError::SerdeJson(ref e) => Some(e),
            #[cfg(feature = "websocket")]
            FugleError::Tungstenite(ref e) => Some(e),
            FugleError::Ureq(ref e) => Some(e),
            FugleError::StdIO(ref e) => Some(e),
            FugleError::General(ref _e) => None,
            FugleError::Unknown(ref _e) => None,
            FugleError::Unauthorized => None,
            FugleError::RateLimitExceeded => None,
            FugleError::ResourceNotFound => None,
            FugleError::MpscSendError => None,
            FugleError::MpscRecvError(ref e) => Some(e),
        }
    }
}

impl From<std::sync::mpsc::RecvError> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: std::sync::mpsc::RecvError) -> FugleError {
        FugleError::MpscRecvError(err)
    }
}

impl From<std::io::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: std::io::Error) -> FugleError {
        FugleError::StdIO(err)
    }
}

impl From<ureq::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: ureq::Error) -> FugleError {
        FugleError::Ureq(Box::new(err))
    }
}

#[cfg(feature = "websocket")]
impl From<tungstenite::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: tungstenite::Error) -> FugleError {
        FugleError::Tungstenite(err)
    }
}

impl From<serde_json::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: serde_json::Error) -> FugleError {
        FugleError::SerdeJson(err)
    }
}

impl From<ErrorResponse> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
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
