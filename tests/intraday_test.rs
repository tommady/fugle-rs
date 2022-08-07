use fugle::{
    errors::FugleError,
    http::intraday::{ChartRequest, DealtsRequest, MetaRequest, QuoteRequest, VolumesRequest},
    http::RestfulBuilder,
};
mod util;

#[test]
fn test_intraday_chart_pass() {
    let it = RestfulBuilder::new().read_timeout_sec(3).build().unwrap();
    let chart = it.call(ChartRequest::new()).unwrap();
    assert_eq!(chart.data.info.symbol_id, "2884");
    assert_eq!(chart.data.info.typ, "EQUITY");

    let chart = it.call(ChartRequest::default().odd_lot(true)).unwrap();
    assert_eq!(chart.data.info.symbol_id, "2884");
    assert_eq!(chart.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_chart_400_failed() {
    let it = RestfulBuilder::default().build().unwrap();
    assert_err!(
        it.call(ChartRequest::new().symbol_id("")),
        Err(FugleError::General(_))
    )
}

#[test]
fn test_intraday_chart_401_failed() {
    let it = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(it.call(ChartRequest::new()), Err(FugleError::Unauthorized))
}

#[test]
fn test_intraday_quote_pass() {
    let it = RestfulBuilder::new().read_timeout_sec(3).build().unwrap();
    let quote = it.call(QuoteRequest::new()).unwrap();
    assert_eq!(quote.data.info.symbol_id, "2884");
    assert_eq!(quote.data.info.typ, "EQUITY");

    let quote = it.call(QuoteRequest::default().odd_lot(true)).unwrap();
    assert_eq!(quote.data.info.symbol_id, "2884");
    assert_eq!(quote.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_quote_400_failed() {
    let it = RestfulBuilder::default().build().unwrap();
    assert_err!(
        it.call(QuoteRequest::new().symbol_id("")),
        Err(FugleError::General(_))
    )
}

#[test]
fn test_intraday_quote_401_failed() {
    let it = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(it.call(QuoteRequest::new()), Err(FugleError::Unauthorized))
}

#[test]
fn test_intraday_meta_pass() {
    let it = RestfulBuilder::new().read_timeout_sec(3).build().unwrap();
    let meta = it.call(MetaRequest::new()).unwrap();
    assert_eq!(meta.data.info.symbol_id, "2884");
    assert_eq!(meta.data.info.typ, "EQUITY");

    let meta = it.call(MetaRequest::default().odd_lot(true)).unwrap();
    assert_eq!(meta.data.info.symbol_id, "2884");
    assert_eq!(meta.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_meta_400_failed() {
    let it = RestfulBuilder::default().build().unwrap();
    assert_err!(
        it.call(MetaRequest::new().symbol_id("")),
        Err(FugleError::General(_))
    )
}

#[test]
fn test_intraday_meta_401_failed() {
    let it = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(it.call(MetaRequest::new()), Err(FugleError::Unauthorized))
}

#[test]
fn test_intraday_dealts_pass() {
    let it = RestfulBuilder::new().read_timeout_sec(3).build().unwrap();
    let dealts = it.call(DealtsRequest::new().limit(9).offset(0)).unwrap();
    assert_eq!(dealts.data.info.symbol_id, "2884");
    assert_eq!(dealts.data.info.typ, "EQUITY");

    let dealts = it
        .call(DealtsRequest::default().odd_lot(true).limit(9).offset(0))
        .unwrap();
    assert_eq!(dealts.data.info.symbol_id, "2884");
    assert_eq!(dealts.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_dealts_400_failed() {
    let it = RestfulBuilder::default().build().unwrap();
    assert_err!(
        it.call(DealtsRequest::new().symbol_id("")),
        Err(FugleError::General(_))
    )
}

#[test]
fn test_intraday_dealts_401_failed() {
    let it = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(it.call(DealtsRequest::new()), Err(FugleError::Unauthorized))
}

#[test]
fn test_intraday_volumes_pass() {
    let it = RestfulBuilder::new().read_timeout_sec(3).build().unwrap();
    let volumes = it.call(VolumesRequest::new()).unwrap();
    assert_eq!(volumes.data.info.symbol_id, "2884");
    assert_eq!(volumes.data.info.typ, "EQUITY");

    let volumes = it.call(VolumesRequest::default().odd_lot(true)).unwrap();
    assert_eq!(volumes.data.info.symbol_id, "2884");
    assert_eq!(volumes.data.info.typ, "ODDLOT");
}

#[test]
fn test_intraday_volumes_400_failed() {
    let it = RestfulBuilder::default().build().unwrap();
    assert_err!(
        it.call(VolumesRequest::new().symbol_id("")),
        Err(FugleError::General(_))
    )
}

#[test]
fn test_intraday_volumes_401_failed() {
    let it = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(
        it.call(VolumesRequest::new()),
        Err(FugleError::Unauthorized)
    )
}

#[test]
fn test_error_rate_limit_exceeded() {
    let it = RestfulBuilder::new().build().unwrap();
    for _ in 0..9 {
        let res = it.call(DealtsRequest::new());
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => panic!("error: {}", e),
            },
        }
    }
}
