use serde::{de, Deserialize, Deserializer, Serialize};
use time::{Date, PrimitiveDateTime};

use crate::errors::FugleError;

pub type Result<T> = std::result::Result<T, FugleError>;

pub fn de_date<'de, D>(deserializer: D) -> std::result::Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format =
        time::format_description::parse("[year]-[month]-[day]").map_err(de::Error::custom)?;
    Date::parse(&s, &format).map_err(de::Error::custom)
}

pub fn de_primitive_date_time<'de, D>(
    deserializer: D,
) -> std::result::Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    PrimitiveDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        .map_err(de::Error::custom)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Info {
    #[serde(deserialize_with = "de_primitive_date_time")]
    pub last_updated_at: PrimitiveDateTime,
    #[serde(deserialize_with = "de_date")]
    pub date: Date,
    pub symbol_id: String,
    pub country_code: String,
    pub time_zone: String,
    pub exchange: String,
    pub market: String,
    #[serde(rename = "type")]
    pub typ: String,
}

impl Default for Info {
    fn default() -> Info {
        Info {
            last_updated_at: PrimitiveDateTime::MIN,
            date: Date::MIN,
            symbol_id: "".to_string(),
            country_code: "".to_string(),
            time_zone: "".to_string(),
            exchange: "".to_string(),
            market: "".to_string(),
            typ: "".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_info_default() {
        let i = Info::default();
        assert_eq!(i.last_updated_at, PrimitiveDateTime::MIN);
        assert_eq!(i.date, Date::MIN);
        assert_eq!(i.symbol_id, "".to_string());
        assert_eq!(i.country_code, "".to_string());
        assert_eq!(i.time_zone, "".to_string());
        assert_eq!(i.exchange, "".to_string());
        assert_eq!(i.market, "".to_string());
        assert_eq!(i.typ, "".to_string());
    }
}
