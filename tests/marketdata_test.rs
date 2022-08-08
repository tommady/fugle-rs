use fugle::{
    errors::FugleError,
    http::{marketdata::CandlesRequest, RestfulBuilder},
};
mod util;

#[test]
fn test_marketdata_candles_pass() {
    let client = RestfulBuilder::default().build().unwrap();
    let candles = client
        .call(CandlesRequest::new().from("2022-08-01").to("2022-08-08"))
        .unwrap();
    assert_eq!(candles.symbol_id, "2884");
    assert_eq!(candles.typ, "EQUITY");
}

// fugle marketdata candles endpoint will goes into 401 if
// 1. token or
// 2. symbol_id
// not provided, not like intraday endpoints have more richable error status
#[test]
fn test_marketdata_candles_401_failed() {
    let client = RestfulBuilder::new().token("").build().unwrap();
    assert_err!(
        client.call(CandlesRequest::default()),
        Err(FugleError::Unauthorized)
    );

    let client = RestfulBuilder::new().build().unwrap();
    assert_err!(
        client.call(CandlesRequest::default().symbol_id("")),
        Err(FugleError::Unauthorized)
    );
}

#[test]
fn test_error_rate_limit_exceeded() {
    let client = RestfulBuilder::default().build().unwrap();
    for _ in 0..9 {
        let res = client.call(CandlesRequest::new().from("2022-08-01").to("2022-08-08"));
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => panic!("error: {}", e),
            },
        }
    }
}
