use std::time::Duration;

use fugle::websocket::IntradayBuilder;
use serial_test::serial;
use tokio::time::sleep as async_sleep;

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
#[serial]
#[cfg(feature = "websocket")]
fn test_intraday_chart_failed() {
    let mut ws = IntradayBuilder::default()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.chart().is_err());
}

#[test]
#[serial]
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
#[serial]
#[cfg(feature = "websocket")]
fn test_intraday_meta_failed() {
    let mut ws = IntradayBuilder::default()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.meta().is_err());
}

#[test]
#[serial]
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
#[serial]
#[cfg(feature = "websocket")]
fn test_intraday_quote_failed() {
    let mut ws = IntradayBuilder::default()
        .symbol_id("2884")
        .odd_lot()
        .token("")
        .build();
    assert!(ws.quote().is_err());
}

#[test]
#[serial]
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

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_chart_failed() {
    let mut ws = IntradayBuilder::default()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.async_chart().await.is_err());
}

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_chart_pass() {
    util::async_timeout_after(Duration::from_secs(3), move || async move {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let mut rx = ws.async_chart().await.unwrap();
        async_sleep(Duration::from_secs(1)).await;
        let chart = rx.recv().await.unwrap();
        assert_eq!(chart.data.info.symbol_id, "2884");
        assert_eq!(chart.data.info.typ, "EQUITY");
    })
    .await
}

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_meta_failed() {
    let mut ws = IntradayBuilder::default()
        .odd_lot()
        .token("")
        .symbol_id("2884")
        .build();
    assert!(ws.async_meta().await.is_err());
}

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_meta_pass() {
    util::async_timeout_after(Duration::from_secs(3), move || async move {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let mut rx = ws.async_meta().await.unwrap();
        async_sleep(Duration::from_secs(1)).await;
        let meta = rx.recv().await.unwrap();
        assert_eq!(meta.data.info.symbol_id, "2884");
        assert_eq!(meta.data.info.typ, "EQUITY");
    })
    .await
}

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_quote_failed() {
    let mut ws = IntradayBuilder::default()
        .symbol_id("2884")
        .odd_lot()
        .token("")
        .build();
    assert!(ws.async_quote().await.is_err());
}

#[tokio::test]
#[serial]
#[cfg(feature = "async-websocket")]
async fn test_intraday_async_quote_pass() {
    util::async_timeout_after(Duration::from_secs(3), move || async move {
        let mut ws = IntradayBuilder::new().symbol_id("2884").build();
        let mut rx = ws.async_quote().await.unwrap();
        async_sleep(Duration::from_secs(1)).await;
        let quote = rx.recv().await.unwrap();
        assert_eq!(quote.data.info.symbol_id, "2884");
        assert_eq!(quote.data.info.typ, "EQUITY");
    })
    .await
}
