use crate::schema::{ChartResponse, DealtsResponse, ErrorResponse, MetaResponse, QuoteResponse};
use std::error;

const INTRADAY_CHART: &str = "https://api.fugle.tw/realtime/v0/intraday/chart";
const INTRADAY_QUOTE: &str = "https://api.fugle.tw/realtime/v0/intraday/quote";
const INTRADAY_META: &str = "https://api.fugle.tw/realtime/v0/intraday/meta";
const INTRADAY_DEALTS: &str = "https://api.fugle.tw/realtime/v0/intraday/dealts";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// [Endpoint]: https://developer.fugle.tw/document/intraday/chart
///
/// Fetching the current drawing data.
pub fn intraday_chart(id: &str, token: &str) -> Result<ChartResponse> {
    let resp = reqwest::blocking::get(
        format!("{}?symbolId={}&apiToken={}", INTRADAY_CHART, id, token).as_str(),
    )?;

    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        let err: ErrorResponse = resp.json()?;
        Err(err.into())
    }
}

/// [Endpoint]: https://developer.fugle.tw/document/intraday/quote
///
/// Fetching the current status and statistics.
pub fn intraday_quote(id: &str, token: &str) -> Result<QuoteResponse> {
    let resp = reqwest::blocking::get(
        format!("{}?symbolId={}&apiToken={}", INTRADAY_QUOTE, id, token).as_str(),
    )?;

    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        let err: ErrorResponse = resp.json()?;
        Err(err.into())
    }
}

/// [Endpoint]: https://developer.fugle.tw/document/intraday/meta
///
/// Fetching today's basic informations.
pub fn intraday_meta(id: &str, token: &str) -> Result<MetaResponse> {
    let resp = reqwest::blocking::get(
        format!("{}?symbolId={}&apiToken={}", INTRADAY_META, id, token).as_str(),
    )?;

    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        let err: ErrorResponse = resp.json()?;
        Err(err.into())
    }
}

/// [Endpoint]: https://developer.fugle.tw/document/intraday/dealts
///
/// Fetching today's advantage information.
pub fn intraday_dealts(id: &str, token: &str) -> Result<DealtsResponse> {
    let resp = reqwest::blocking::get(
        format!("{}?symbolId={}&apiToken={}", INTRADAY_DEALTS, id, token).as_str(),
    )?;

    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        let err: ErrorResponse = resp.json()?;
        Err(err.into())
    }
}
