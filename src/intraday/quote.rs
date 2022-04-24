use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::schema::{default_date_time, Info};

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
