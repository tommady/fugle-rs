use fugle::websocket::IntradayBuilder;
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
#[cfg(feature = "websocket")]
fn test_intraday_chart_failed() {
    let mut ws = IntradayBuilder::new()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.chart().is_err());
}

#[test]
#[cfg(feature = "websocket")]
fn test_intraday_chart_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let rx = ws.chart().unwrap();
        let chart = rx.recv().unwrap();
        assert_eq!(chart.data.info.symbol_id, "2884");
        assert_eq!(chart.data.info.typ, "EQUITY");
    })
}

#[test]
#[cfg(feature = "websocket")]
fn test_intraday_meta_failed() {
    let mut ws = IntradayBuilder::new()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.meta().is_err());
}

#[test]
#[cfg(feature = "websocket")]
fn test_intraday_meta_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let rx = ws.meta().unwrap();
        let meta = rx.recv().unwrap();
        assert_eq!(meta.data.info.symbol_id, "2884");
        assert_eq!(meta.data.info.typ, "EQUITY");
    })
}

#[test]
#[cfg(feature = "websocket")]
fn test_intraday_quote_failed() {
    let mut ws = IntradayBuilder::new()
        .symbol_id("2884")
        .odd_lot()
        .token("")
        .build();
    assert!(ws.quote().is_err());
}

#[test]
#[cfg(feature = "websocket")]
fn test_intraday_quote_pass() {
    util::timeout_after(Duration::from_secs(3), || {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let rx = ws.quote().unwrap();
        let quote = rx.recv().unwrap();
        assert_eq!(quote.data.info.symbol_id, "2884");
        assert_eq!(quote.data.info.typ, "EQUITY");
    })
}
