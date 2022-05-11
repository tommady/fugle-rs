use fugle::{errors::FugleError, intraday::IntradayBuilder};
mod util;

#[test]
fn test_intraday_chart_pass() {
    let it = IntradayBuilder::new().read_timeout_sec(3).build();
    let chart = it.chart("2884").call().unwrap();
    assert_eq!(chart.data.info.symbol_id, "2884");
    assert_eq!(chart.data.info.typ, "EQUITY");

    let chart = it.chart("2884").odd_lot(true).call().unwrap();
    assert_eq!(chart.data.info.symbol_id, "2884");
    assert_eq!(chart.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_chart_400_failed() {
    let it = IntradayBuilder::default().build();
    assert_err!(it.chart("").call(), Err(FugleError::General(_)));
}

#[test]
fn test_intraday_chart_401_failed() {
    let it = IntradayBuilder::new().token("").build();
    assert_err!(it.chart("2884").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_intraday_quote_pass() {
    let it = IntradayBuilder::new().read_timeout_sec(3).build();
    let quote = it.quote("2884").call().unwrap();
    assert_eq!(quote.data.info.symbol_id, "2884");
    assert_eq!(quote.data.info.typ, "EQUITY");

    let quote = it.quote("2884").odd_lot(true).call().unwrap();
    assert_eq!(quote.data.info.symbol_id, "2884");
    assert_eq!(quote.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_quote_400_failed() {
    let it = IntradayBuilder::default().build();
    assert_err!(it.quote("").call(), Err(FugleError::General(_)));
}

#[test]
fn test_intraday_quote_401_failed() {
    let it = IntradayBuilder::new().token("").build();
    assert_err!(it.quote("2884").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_intraday_meta_pass() {
    let it = IntradayBuilder::new().read_timeout_sec(3).build();
    let meta = it.meta("2884").call().unwrap();
    assert_eq!(meta.data.info.symbol_id, "2884");
    assert_eq!(meta.data.info.typ, "EQUITY");

    let meta = it.meta("2884").odd_lot(true).call().unwrap();
    assert_eq!(meta.data.info.symbol_id, "2884");
    assert_eq!(meta.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_meta_400_failed() {
    let it = IntradayBuilder::default().build();
    assert_err!(it.meta("").call(), Err(FugleError::General(_)));
}

#[test]
fn test_intraday_meta_401_failed() {
    let it = IntradayBuilder::new().token("").build();
    assert_err!(it.meta("2884").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_intraday_dealts_pass() {
    let it = IntradayBuilder::new().read_timeout_sec(3).build();
    let dealts = it.dealts("2884").limit(9).offset(0).call().unwrap();
    assert_eq!(dealts.data.info.symbol_id, "2884");
    assert_eq!(dealts.data.info.typ, "EQUITY");

    let dealts = it
        .dealts("2884")
        .odd_lot(true)
        .limit(9)
        .offset(0)
        .call()
        .unwrap();
    assert_eq!(dealts.data.info.symbol_id, "2884");
    assert_eq!(dealts.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_dealts_400_failed() {
    let it = IntradayBuilder::default().build();
    assert_err!(it.dealts("").call(), Err(FugleError::General(_)));
}

#[test]
fn test_intraday_dealts_401_failed() {
    let it = IntradayBuilder::new().token("").build();
    assert_err!(it.dealts("2884").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_intraday_volumes_pass() {
    let it = IntradayBuilder::new().read_timeout_sec(3).build();
    let volumes = it.volumes("2884").call().unwrap();
    assert_eq!(volumes.data.info.symbol_id, "2884");
    assert_eq!(volumes.data.info.typ, "EQUITY");

    let volumes = it.volumes("2884").odd_lot(true).call().unwrap();
    assert_eq!(volumes.data.info.symbol_id, "2884");
    assert_eq!(volumes.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_volumes_400_failed() {
    let it = IntradayBuilder::default().build();
    assert_err!(it.volumes("").call(), Err(FugleError::General(_)));
}

#[test]
fn test_intraday_volumes_401_failed() {
    let it = IntradayBuilder::new().token("").build();
    assert_err!(it.volumes("2884").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_error_rate_limit_exceeded() {
    let it = IntradayBuilder::new().build();
    for _ in 0..9 {
        let res = it.dealts("2884").call();
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => panic!("error: {}", e),
            },
        }
    }
}