use crate::{
    http::{Query, Request},
    schema::CandlesResponse,
};

/// [Endpoint](https://developer.fugle.tw/docs/data/marketdata/candles)
///
/// Fetching history stock information.
///
pub struct CandlesRequest<'a> {
    from: &'a str,
    to: &'a str,
    symbol_id: &'a str,
    fields: [CandleField; 7],
}

impl Default for CandlesRequest<'_> {
    fn default() -> Self {
        CandlesRequest::new()
    }
}

#[derive(Copy, Clone)]
pub enum CandleField {
    NoValue,
    Open,
    High,
    Low,
    Close,
    Volume,
    Turnover,
    Change,
}

impl<'a> CandlesRequest<'a> {
    pub fn new() -> Self {
        CandlesRequest {
            symbol_id: "2884",
            from: "",
            to: "",
            fields: [CandleField::NoValue; 7],
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

    pub fn add_field(mut self, field: CandleField) -> Self {
        match field {
            CandleField::Open => self.fields[0] = CandleField::Open,
            CandleField::High => self.fields[1] = CandleField::High,
            CandleField::Low => self.fields[2] = CandleField::Low,
            CandleField::Close => self.fields[3] = CandleField::Close,
            CandleField::Volume => self.fields[4] = CandleField::Volume,
            CandleField::Turnover => self.fields[5] = CandleField::Turnover,
            CandleField::Change => self.fields[6] = CandleField::Change,
            CandleField::NoValue => {}
        }
        self
    }

    pub fn remove_field(mut self, field: CandleField) -> Self {
        match field {
            CandleField::Open => self.fields[0] = CandleField::NoValue,
            CandleField::High => self.fields[1] = CandleField::NoValue,
            CandleField::Low => self.fields[2] = CandleField::NoValue,
            CandleField::Close => self.fields[3] = CandleField::NoValue,
            CandleField::Volume => self.fields[4] = CandleField::NoValue,
            CandleField::Turnover => self.fields[5] = CandleField::NoValue,
            CandleField::Change => self.fields[6] = CandleField::NoValue,
            CandleField::NoValue => {}
        }
        self
    }
}

impl Request for CandlesRequest<'_> {
    const REQUEST_URL: &'static str = "https://api.fugle.tw/marketdata/v0.3/candles";
    type Response = CandlesResponse;

    fn queries(&self) -> Vec<Query> {
        let mut ret = vec![Query {
            param: "symbolId".to_string(),
            value: self.symbol_id.to_string(),
        }];

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

        for field in self.fields {
            match field {
                CandleField::Open => fields.push("open"),
                CandleField::High => fields.push("high"),
                CandleField::Low => fields.push("low"),
                CandleField::Close => fields.push("close"),
                CandleField::Volume => fields.push("volume"),
                CandleField::Turnover => fields.push("turnover"),
                CandleField::Change => fields.push("change"),
                CandleField::NoValue => {}
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
