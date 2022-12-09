use serde::{Deserialize, Serialize};
use time::Date;

use crate::schema::de_date;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Candle {
    #[serde(deserialize_with = "de_date")]
    pub date: Date,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub turnover: u64,
    pub change: f64,
}

impl Default for Candle {
    fn default() -> Candle {
        Candle {
            date: Date::MIN,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0,
            turnover: 0,
            change: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CandlesResponse {
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
            candles: vec![],
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
        assert_eq!(c.open, 0.0);
        assert_eq!(c.high, 0.0);
        assert_eq!(c.low, 0.0);
        assert_eq!(c.close, 0.0);
        assert_eq!(c.volume, 0);
        assert_eq!(c.turnover, 0);
        assert_eq!(c.change, 0.0);
    }
}
