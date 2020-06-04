use fugle::{listener, schema::Response};
use std::sync::mpsc;

#[test]
fn test_intraday_chart_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx.clone());
    assert!(lis.chart("2884").is_err());
}

#[test]
fn test_intraday_chart_pass() {
    let (tx, rx) = mpsc::channel();
    let mut lis = listener::Intraday::new("demo", tx.clone());
    assert!(lis.chart("2884").is_ok());
    let res = rx.recv();
    assert!(res.is_ok());
    if let Response::ChartResponse(c) = res.unwrap() {
        assert_eq!(c.data.info.symbol_id, "2884");
    }
}

#[test]
fn test_intraday_meta_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx.clone());
    assert!(lis.meta("2884").is_err());
}

#[test]
fn test_intraday_meta_pass() {
    let (tx, rx) = mpsc::channel();
    let mut lis = listener::Intraday::new("demo", tx.clone());
    assert!(lis.chart("2884").is_ok());
    let res = rx.recv();
    assert!(res.is_ok());
    if let Response::MetaResponse(m) = res.unwrap() {
        assert_eq!(m.data.info.symbol_id, "2884");
    }
}

#[test]
fn test_intraday_quote_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx.clone());
    assert!(lis.quote("2884").is_err());
}

#[test]
fn test_intraday_quote_pass() {
    let (tx, rx) = mpsc::channel();
    let mut lis = listener::Intraday::new("demo", tx.clone());
    assert!(lis.chart("2884").is_ok());
    let res = rx.recv();
    assert!(res.is_ok());
    if let Response::QuoteResponse(q) = res.unwrap() {
        assert_eq!(q.data.info.symbol_id, "2884");
    }
}
