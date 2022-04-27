use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::errors::FugleError;

pub type Result<T> = std::result::Result<T, FugleError>;

#[cfg_attr(coverage, no_coverage)]
pub(crate) fn default_naive_date() -> NaiveDate {
    NaiveDate::from_num_days_from_ce(1)
}

#[cfg_attr(coverage, no_coverage)]
pub(crate) fn default_date_time() -> DateTime<FixedOffset> {
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
