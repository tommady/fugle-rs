use crate::{
    http::{Query, Request},
    schema::CandlesResponse,
};

use std::{fmt, slice::Iter};

/// [Endpoint](https://developer.fugle.tw/docs/data/marketdata/candles)
///
/// Fetching history stock information.
///
pub struct CandlesRequest<'a> {
    from: &'a str,
    to: &'a str,
    symbol_id: &'a str,
    fields: u8,
}

impl Default for CandlesRequest<'_> {
    fn default() -> Self {
        CandlesRequest::new()
    }
}

pub enum CandleField {
    Open,
    High,
    Low,
    Close,
    Volume,
    Turnover,
    Change,
}

impl fmt::Display for CandleField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CandleField::Open => write!(f, "open"),
            CandleField::High => write!(f, "high"),
            CandleField::Low => write!(f, "low"),
            CandleField::Close => write!(f, "close"),
            CandleField::Volume => write!(f, "volume"),
            CandleField::Turnover => write!(f, "turnover"),
            CandleField::Change => write!(f, "change"),
        }
    }
}

impl CandleField {
    fn value(&self) -> u8 {
        match *self {
            CandleField::Open => 1 << 1,
            CandleField::High => 1 << 2,
            CandleField::Low => 1 << 3,
            CandleField::Close => 1 << 4,
            CandleField::Volume => 1 << 5,
            CandleField::Turnover => 1 << 6,
            CandleField::Change => 1 << 7,
        }
    }

    fn iterator() -> Iter<'static, CandleField> {
        static FIELDS: [CandleField; 7] = [
            CandleField::Open,
            CandleField::High,
            CandleField::Low,
            CandleField::Close,
            CandleField::Volume,
            CandleField::Turnover,
            CandleField::Change,
        ];
        FIELDS.iter()
    }
}

impl<'a> CandlesRequest<'a> {
    pub fn new() -> Self {
        CandlesRequest {
            symbol_id: "2884",
            from: "",
            to: "",
            fields: 0,
        }
    }

    pub fn symbol_id(mut self, symbol_id: &'a str) -> Self {
        self.symbol_id = symbol_id;
        self
    }

    pub fn from(mut self, day: &'a str) -> Self {
        self.from = day;
        self
    }

    pub fn to(mut self, day: &'a str) -> Self {
        self.to = day;
        self
    }

    pub fn set_field(mut self, field: CandleField) -> Self {
        self.fields |= field.value();
        self
    }

    pub fn unset_field(mut self, field: CandleField) -> Self {
        self.fields &= field.value();
        self.fields ^= field.value();
        self
    }
}

impl Request for CandlesRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/marketdata/v0.3/candles";
    type Response = CandlesResponse;

    fn queries(&self) -> Vec<Query> {
        let mut ret = Vec::with_capacity(4);

        if !self.symbol_id.is_empty() {
            ret.push(Query {
                param: "symbolId".to_string(),
                value: self.symbol_id.to_string(),
            })
        }
        if !self.from.is_empty() {
            ret.push(Query {
                param: "from".to_string(),
                value: self.from.to_string(),
            })
        }
        if !self.to.is_empty() {
            ret.push(Query {
                param: "to".to_string(),
                value: self.to.to_string(),
            })
        }

        let mut fields = vec![];
        for field in CandleField::iterator() {
            if self.fields & field.value() != 0 {
                fields.push(field.to_string());
            }
        }
        if !fields.is_empty() {
            ret.push(Query {
                param: "fields".to_string(),
                value: fields.join(","),
            })
        }
        ret
    }
}
