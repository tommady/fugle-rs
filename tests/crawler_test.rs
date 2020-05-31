use fugle::crawler;

#[test]
fn test_intraday_chart_pass() {
    let res = crawler::intraday_chart("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn test_intraday_chart_failed() {
    let res = crawler::intraday_chart("", "");
    assert!(!res.is_ok());
}

#[test]
fn test_intraday_quote_pass() {
    let res = crawler::intraday_quote("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn test_intraday_quote_failed() {
    let res = crawler::intraday_quote("", "");
    assert!(!res.is_ok());
}

#[test]
fn test_intraday_meta_pass() {
    let res = crawler::intraday_meta("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn test_intraday_meta_failed() {
    let res = crawler::intraday_meta("", "");
    assert!(!res.is_ok());
}
