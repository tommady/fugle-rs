use fugle::crawler;
use fugle::schema::{FugleError, Response};
mod macros;

#[test]
fn test_intraday_chart_pass() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.chart("2884").call();
    match res {
        Ok(v) => {
            let chart = fetch_enum!(Response::Chart, v);
            assert_eq!(chart.data.info.symbol_id, "2884");
        }
        Err(e) => assert!(false, "error: {}", e.to_string()),
    };
}

#[test]
fn test_intraday_chart_failed() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.chart("").call();
    assert!(res.is_err());
}

#[test]
fn test_intraday_quote_pass() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.quote("2884").call();
    match res {
        Ok(v) => {
            let quote = fetch_enum!(Response::Quote, v);
            assert_eq!(quote.data.info.symbol_id, "2884");
        }
        Err(e) => assert!(false, "error: {}", e.to_string()),
    };
}

#[test]
fn test_intraday_quote_failed() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.quote("").call();
    assert!(res.is_err());
}

#[test]
fn test_intraday_meta_pass() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.meta("2884").call();
    match res {
        Ok(v) => {
            let meta = fetch_enum!(Response::Meta, v);
            assert_eq!(meta.data.info.symbol_id, "2884");
        }
        Err(e) => assert!(false, "error: {}", e.to_string()),
    };
}

#[test]
fn test_intraday_meta_failed() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.meta("").call();
    assert!(res.is_err());
}

#[test]
fn test_intraday_deals_pass() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.dealts("2884").call();
    match res {
        Ok(v) => {
            let meta = fetch_enum!(Response::Dealts, v);
            assert_eq!(meta.data.info.symbol_id, "2884");
        }
        Err(e) => assert!(false, "error: {}", e.to_string()),
    };
}

#[test]
fn test_intraday_dealts_failed() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.dealts("").call();
    assert!(res.is_err());
}

// the fugle api returns 403 with message Forbidden in every errors now...
// this case was testing 401 unauthorized.
#[test]
fn test_error_rate_limit_exceeded() {
    let it = crawler::IntradayBuilder::new().build();
    let res = it.dealts("2884").call();
    match res {
        Ok(v) => assert!(false, "ok: {:?}", v),
        Err(e) => match e {
            FugleError::RateLimitExceeded => assert!(true),
            _ => assert!(false, "error: {}", e),
        },
    }
}
