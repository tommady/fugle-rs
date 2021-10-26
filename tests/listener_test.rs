use fugle::{listener, schema::Response};
use std::sync::mpsc;
use std::time::Duration;
mod util;

// be aware of websocket testings
// running pass tests by cargo test -- --ignored
//
// there are two cases will causing the pass tests blocking
// 1. fugle websocket server is down or suspend
//    you can check on this website
//    https://developer.fugle.tw/document/status
//
// 2. the Taiwan stock market is closing
//    in this case fugle websocket server won't send any data
//    while you connected into the server

#[test]
fn test_intraday_chart_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx);
    assert!(lis.chart("2884", true).is_err());
}

#[test]
fn test_intraday_chart_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let (tx, rx) = mpsc::channel();
        let mut lis = listener::Intraday::new("demo", tx);
        assert!(lis.chart("2884", false).is_ok());
        let chart = fetch_enum!(Response::Chart, rx.recv().unwrap());
        assert_eq!(chart.data.info.symbol_id, "2884");
        assert_eq!(chart.data.info.typ, "EQUITY");
    })
}

#[test]
fn test_intraday_meta_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx);
    assert!(lis.meta("2884", true).is_err());
}

#[test]
fn test_intraday_meta_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let (tx, rx) = mpsc::channel();
        let mut lis = listener::Intraday::new("demo", tx);
        assert!(lis.meta("2884", false).is_ok());
        let meta = fetch_enum!(Response::Meta, rx.recv().unwrap());
        assert_eq!(meta.data.info.symbol_id, "2884");
        assert_eq!(meta.data.info.typ, "EQUITY");
    })
}

#[test]
fn test_intraday_quote_failed() {
    let (tx, _) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx);
    assert!(lis.quote("2884", true).is_err());
}

#[test]
fn test_intraday_quote_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let (tx, rx) = mpsc::channel();
        let mut lis = listener::Intraday::new("demo", tx);
        assert!(lis.quote("2884", false).is_ok());
        let quote = fetch_enum!(Response::Quote, rx.recv().unwrap());
        assert_eq!(quote.data.info.symbol_id, "2884");
        assert_eq!(quote.data.info.typ, "EQUITY");
    })
}
