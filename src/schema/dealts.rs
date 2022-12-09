use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::schema::{de_primitive_date_time, Info};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Dealt {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub at: PrimitiveDateTime,
    pub bid: Decimal,
    pub ask: Decimal,
    pub price: Decimal,
    pub volume: u64,
    pub serial: u64,
}

impl Default for Dealt {
    fn default() -> Dealt {
        Dealt {
            at: PrimitiveDateTime::MIN,
            bid: Decimal::new(0, 2),
            ask: Decimal::new(0, 2),
            price: Decimal::new(0, 2),
            volume: 0,
            serial: 0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DealtsData {
    pub info: Info,
    pub dealts: Vec<Dealt>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DealtsResponse {
    pub api_version: String,
    pub data: DealtsData,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dealt_default() {
        let d = Dealt::default();
        assert_eq!(d.at, PrimitiveDateTime::MIN);
        assert_eq!(d.bid, Decimal::new(0, 2));
        assert_eq!(d.ask, Decimal::new(0, 2));
        assert_eq!(d.price, Decimal::new(0, 2));
        assert_eq!(d.volume, 0);
        assert_eq!(d.serial, 0);
    }
}
