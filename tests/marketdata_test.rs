use fugle::{errors::FugleError, marketdata::MarketdataBuilder};
mod util;

#[test]
fn test_marketdata_candles_pass() {
    let mk = MarketdataBuilder::default().read_timeout_sec(3).build();
    let candles = mk
        .candles("2884")
        .from("2022-04-21")
        .to("2022-04-28")
        .call()
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
    let mk = MarketdataBuilder::new().token("").build();
    assert_err!(mk.candles("2884").call(), Err(FugleError::Unauthorized));

    let mk = MarketdataBuilder::new().token("demo").build();
    assert_err!(mk.candles("").call(), Err(FugleError::Unauthorized));
}

#[test]
fn test_error_rate_limit_exceeded() {
    let mk = MarketdataBuilder::new().build();
    for _ in 0..9 {
        let res = mk.candles("2884").call();
        match res {
            Ok(_) => continue,
            Err(e) => match e {
                FugleError::RateLimitExceeded => break,
                _ => panic!("error: {}", e),
            },
        }
    }
}
