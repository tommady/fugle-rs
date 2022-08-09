use fugle::{
    errors::FugleError,
    http::{marketdata::CandlesRequest, RestfulBuilder},
};
mod util;

#[test]
fn test_marketdata_candles_pass() {
    let client = RestfulBuilder::default()
        .read_timeout_sec(3)
        .build()
        .unwrap();
    let candles = client
        .call(CandlesRequest::new().from("2022-08-01").to("2022-08-08"))
        .unwrap();
    assert_eq!(candles.symbol_id, "2884");
    assert_eq!(candles.typ, "EQUITY");
}

#[tokio::test]
async fn test_marketdata_async_candles_pass() {
    let client = RestfulBuilder::default()
        .read_timeout_sec(3)
        .build_async()
        .unwrap();
    let candles = client
        .call(CandlesRequest::new().from("2022-08-01").to("2022-08-08"))
        .await
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

#[tokio::test]
async fn test_marketdata_async_candles_401_failed() {
    let client = RestfulBuilder::new().token("").build_async().unwrap();
    assert_err!(
        client.call(CandlesRequest::default()).await,
        Err(FugleError::Unauthorized)
    );

    let client = RestfulBuilder::new().build_async().unwrap();
    assert_err!(
        client.call(CandlesRequest::default().symbol_id("")).await,
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

#[tokio::test]
async fn test_error_async_rate_limit_exceeded() {
    let client = RestfulBuilder::default().build_async().unwrap();
    for _ in 0..9 {
        let res = client
            .call(CandlesRequest::new().from("2022-08-01").to("2022-08-08"))
            .await;
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => panic!("error: {}", e),
            },
        }
    }
}
