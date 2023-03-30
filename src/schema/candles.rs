use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::schema::de_date;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Candle {
    #[serde(deserialize_with = "de_date")]
    pub date: Date,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: u64,
    pub turnover: u64,
    pub change: Decimal,
}

impl Default for Candle {
    fn default() -> Candle {
        Candle {
            date: Date::MIN,
            open: Decimal::new(0, 2),
            high: Decimal::new(0, 2),
            low: Decimal::new(0, 2),
            close: Decimal::new(0, 2),
            volume: 0,
            turnover: 0,
            change: Decimal::new(0, 2),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CandlesResponse {
    #[serde(rename = "symbol")]
    pub symbol_id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub exchange: String,
    pub market: String,
    pub candles: Vec<Candle>,
}

impl Default for CandlesResponse {
    fn default() -> CandlesResponse {
        CandlesResponse {
            symbol_id: "".to_string(),
            typ: "".to_string(),
            exchange: "".to_string(),
            market: "".to_string(),
            candles: Vec::with_capacity(0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_candle_default() {
        let c = Candle::default();
        assert_eq!(c.date, Date::MIN);
        assert_eq!(c.open, Decimal::new(0, 2));
        assert_eq!(c.high, Decimal::new(0, 2));
        assert_eq!(c.low, Decimal::new(0, 2));
        assert_eq!(c.close, Decimal::new(0, 2));
        assert_eq!(c.volume, 0);
        assert_eq!(c.turnover, 0);
        assert_eq!(c.change, Decimal::new(0, 2));
    }
}
