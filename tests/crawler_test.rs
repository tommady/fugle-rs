use fugle::crawler;
use fugle::schema::{FugleError, Response};
mod util;

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

// from this time now... i don't know why 403 is not responding on demo token...
// so this testcase cannot be tested right now...
//
// you can test with below command line
// for run in {1..100}; do curl -X GET "https://api.fugle.tw/realtime/v0.3/intraday/meta?apiToken=demo&symbolId=2884"; done
// it was responding 403 before...
//
// TODO: remove the for loop when the fugle server start reponding 403 on demo token
#[test]
fn test_error_rate_limit_exceeded() {
    let it = crawler::IntradayBuilder::new().build();
    for _ in 0..9 {
        let res = it.dealts("2884").call();
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => assert!(false, "error: {}", e),
            },
        }
    }
    assert!(true)
}
