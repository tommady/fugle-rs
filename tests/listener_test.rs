use fugle::{listener, schema::Response};
use std::sync::mpsc;
mod macros;

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
    let v = res.unwrap();
    let chart = fetch_enum!(Response::Chart, v);
    assert_eq!(chart.data.info.symbol_id, "2884");
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
    let v = res.unwrap();
    let meta = fetch_enum!(Response::Meta, v);
    assert_eq!(meta.data.info.symbol_id, "2884");
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
    let v = res.unwrap();
    let quote = fetch_enum!(Response::Quote, v);
    assert_eq!(quote.data.info.symbol_id, "2884");
}
