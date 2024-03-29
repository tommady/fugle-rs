use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::schema::{de_primitive_date_time, Info};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteTotal {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
    pub transaction: u64,
    pub trade_value: Decimal,
    pub trade_volume: u64,
    pub trade_volume_at_bid: u64,
    pub trade_volume_at_ask: u64,
    pub bid_orders: u64,
    pub ask_orders: u64,
    pub bid_volume: u64,
    pub ask_volume: u64,
    pub serial: u64,
}

impl Default for QuoteTotal {
    fn default() -> QuoteTotal {
        QuoteTotal {
            at: PrimitiveDateTime::MIN,
            transaction: 0,
            trade_value: Decimal::new(0, 2),
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
#[serde(rename_all = "camelCase", default)]
pub struct QuoteTrial {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
    pub bid: Decimal,
    pub ask: Decimal,
    pub price: Decimal,
    pub volume: u64,
}

impl Default for QuoteTrial {
    fn default() -> QuoteTrial {
        QuoteTrial {
            at: PrimitiveDateTime::MIN,
            bid: Decimal::new(0, 2),
            ask: Decimal::new(0, 2),
            price: Decimal::new(0, 2),
            volume: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteTrade {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
    pub bid: Decimal,
    pub ask: Decimal,
    pub price: Decimal,
    pub volume: u64,
    pub serial: u64,
}

impl Default for QuoteTrade {
    fn default() -> QuoteTrade {
        QuoteTrade {
            at: PrimitiveDateTime::MIN,
            price: Decimal::new(0, 2),
            bid: Decimal::new(0, 2),
            ask: Decimal::new(0, 2),
            volume: 0,
            serial: 0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteBidAsk {
    pub price: Decimal,
    pub volume: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteOrder {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
    pub bids: Vec<QuoteBidAsk>,
    pub asks: Vec<QuoteBidAsk>,
}

impl Default for QuoteOrder {
    fn default() -> QuoteOrder {
        QuoteOrder {
            at: PrimitiveDateTime::MIN,
            bids: Vec::with_capacity(0),
            asks: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuotePrice {
    pub price: Decimal,
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
}

impl Default for QuotePrice {
    fn default() -> QuotePrice {
        QuotePrice {
            at: PrimitiveDateTime::MIN,
            price: Decimal::new(0, 2),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Quote {
    pub is_curbing: bool,
    pub is_curbing_rise: bool,
    pub is_curbing_fall: bool,
    pub is_trial: bool,
    pub is_open_delayed: bool,
    pub is_close_delayed: bool,
    pub is_halting: bool,
    pub is_closed: bool,
    pub total: QuoteTotal,
    pub trial: QuoteTrial,
    pub trade: QuoteTrade,
    pub order: QuoteOrder,
    pub price_high: QuotePrice,
    pub price_low: QuotePrice,
    pub price_open: QuotePrice,
    pub price_avg: QuotePrice,
    pub change: Decimal,
    pub change_percent: Decimal,
    pub amplitude: Decimal,
    pub price_limit: u8,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteData {
    pub info: Info,
    pub quote: Quote,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuoteResponse {
    pub api_version: String,
    pub data: QuoteData,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quote_total_default() {
        let q = QuoteTotal::default();
        assert_eq!(q.at, PrimitiveDateTime::MIN);
        assert_eq!(q.transaction, 0);
        assert_eq!(q.trade_value, Decimal::new(0, 2));
        assert_eq!(q.trade_volume, 0);
        assert_eq!(q.trade_volume_at_bid, 0);
        assert_eq!(q.trade_volume_at_ask, 0);
        assert_eq!(q.bid_orders, 0);
        assert_eq!(q.ask_orders, 0);
        assert_eq!(q.bid_volume, 0);
        assert_eq!(q.ask_volume, 0);
        assert_eq!(q.serial, 0);
    }

    #[test]
    fn test_quote_trial_default() {
        let q = QuoteTrial::default();
        assert_eq!(q.at, PrimitiveDateTime::MIN);
        assert_eq!(q.bid, Decimal::new(0, 2));
        assert_eq!(q.ask, Decimal::new(0, 2));
        assert_eq!(q.price, Decimal::new(0, 2));
        assert_eq!(q.volume, 0);
    }

    #[test]
    fn test_quote_trade_default() {
        let q = QuoteTrade::default();
        assert_eq!(q.at, PrimitiveDateTime::MIN);
        assert_eq!(q.bid, Decimal::new(0, 2));
        assert_eq!(q.ask, Decimal::new(0, 2));
        assert_eq!(q.price, Decimal::new(0, 2));
        assert_eq!(q.volume, 0);
        assert_eq!(q.serial, 0);
    }

    #[test]
    fn test_quote_order_default() {
        let q = QuoteOrder::default();
        assert_eq!(q.at, PrimitiveDateTime::MIN);
        assert_eq!(q.bids.len(), 0);
        assert_eq!(q.asks.len(), 0);
    }

    #[test]
    fn test_quote_price() {
        let q = QuotePrice::default();
        assert_eq!(q.at, PrimitiveDateTime::MIN);
        assert_eq!(q.price, Decimal::new(0, 2));
    }
}
