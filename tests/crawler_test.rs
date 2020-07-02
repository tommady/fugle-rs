use fugle::crawler;

#[test]
fn test_intraday_chart_pass() {
    let res = crawler::intraday_chart("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => assert!(false, e.to_string()),
    };
}

#[test]
fn test_intraday_chart_failed() {
    let res = crawler::intraday_chart("", "");
    assert!(res.is_err());
}

#[test]
fn test_intraday_quote_pass() {
    let res = crawler::intraday_quote("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => assert!(false, e.to_string()),
    };
}

#[test]
fn test_intraday_quote_failed() {
    let res = crawler::intraday_quote("", "");
    assert!(res.is_err());
}

#[test]
fn test_intraday_meta_pass() {
    let res = crawler::intraday_meta("2884", "demo");
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => assert!(false, e.to_string()),
    };
}

#[test]
fn test_intraday_meta_failed() {
    let res = crawler::intraday_meta("", "");
    assert!(res.is_err());
}

#[test]
fn test_intraday_deals_pass() {
    let res = crawler::intraday_dealts("2884", "demo", 0, 0);
    match res {
        Ok(v) => assert_eq!(v.data.info.symbol_id, "2884"),
        Err(e) => assert!(false, e.to_string()),
    };
}

#[test]
fn test_intraday_dealts_failed() {
    let res = crawler::intraday_dealts("", "", 0, 0);
    assert!(res.is_err());
}
