use crate::schema::{
    ChartResponse, DealtsResponse, ErrorResponse, MetaResponse, QuoteResponse, Result,
};

const INTRADAY_CHART: &str = "https://api.fugle.tw/realtime/v0/intraday/chart";
const INTRADAY_QUOTE: &str = "https://api.fugle.tw/realtime/v0/intraday/quote";
const INTRADAY_META: &str = "https://api.fugle.tw/realtime/v0/intraday/meta";
const INTRADAY_DEALTS: &str = "https://api.fugle.tw/realtime/v0/intraday/dealts";

/// [Endpoint]: https://developer.fugle.tw/document/intraday/chart
///
/// Fetching the current drawing data.
pub fn intraday_chart(id: &str, token: &str) -> Result<ChartResponse> {
    let url =
        reqwest::Url::parse_with_params(INTRADAY_CHART, &[("symbolId", id), ("apiToken", token)])?;
    let resp = reqwest::blocking::get(url)?;

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
    let url =
        reqwest::Url::parse_with_params(INTRADAY_QUOTE, &[("symbolId", id), ("apiToken", token)])?;
    let resp = reqwest::blocking::get(url)?;

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
    let url =
        reqwest::Url::parse_with_params(INTRADAY_META, &[("symbolId", id), ("apiToken", token)])?;
    let resp = reqwest::blocking::get(url)?;

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
pub fn intraday_dealts(
    id: &str,
    token: &str,
    limit: usize,
    offset: usize,
) -> Result<DealtsResponse> {
    let url = reqwest::Url::parse_with_params(
        INTRADAY_DEALTS,
        &[
            ("symbolId", id),
            ("apiToken", token),
            ("limit", &limit.to_string()),
            ("offset", &offset.to_string()),
        ],
    )?;
    let resp = reqwest::blocking::get(url)?;

    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        let err: ErrorResponse = resp.json()?;
        Err(err.into())
    }
}
