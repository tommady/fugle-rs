use fugle::schema::*;
use std::fs::File;
use std::path::Path;

#[test]
fn test_chart_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/chart_response.json")).unwrap();
    let res: ChartResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("EQUITY", res.data.info.typ);
    assert_eq!(263, res.data.chart.open.len());
    assert_eq!(263, res.data.chart.high.len());
    assert_eq!(263, res.data.chart.low.len());
    assert_eq!(263, res.data.chart.close.len());
    assert_eq!(263, res.data.chart.unix_timestamp.len());
}

#[test]
fn test_quote_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/quote_response.json")).unwrap();
    let res: QuoteResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("EQUITY", res.data.info.typ);
    assert_eq!(
        (-0.19f64).to_string(),
        res.data.quote.change_percent.to_string()
    );
    assert_eq!(0.38f64.to_string(), res.data.quote.amplitude.to_string());
    assert_eq!(5, res.data.quote.order.bids.len());
    assert_eq!(5, res.data.quote.order.asks.len());
    assert_eq!(
        26.5f64.to_string(),
        res.data.quote.price_high.price.to_string()
    );
    assert_eq!(
        26.4f64.to_string(),
        res.data.quote.price_low.price.to_string()
    );
    assert_eq!(
        26.5f64.to_string(),
        res.data.quote.price_open.price.to_string()
    );
    assert_eq!(
        26.46f64.to_string(),
        res.data.quote.price_avg.price.to_string()
    );
}

#[test]
fn test_meta_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/meta_response.json")).unwrap();
    let res: MetaResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("EQUITY", res.data.info.typ);
    assert_eq!(
        26.5f64.to_string(),
        res.data.meta.price_reference.to_string()
    );
    assert_eq!(
        29.15f64.to_string(),
        res.data.meta.price_high_limit.to_string()
    );
    assert_eq!(
        23.85f64.to_string(),
        res.data.meta.price_low_limit.to_string()
    );
    assert!(res.data.meta.can_day_buy_sell);
    assert_eq!("玉山金", res.data.meta.name_zh_tw);
    assert_eq!("金融保險", res.data.meta.industry_zh_tw);
}

#[test]
fn test_dealts_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/dealts_response.json")).unwrap();
    let res: DealtsResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("EQUITY", res.data.info.typ);
    assert_eq!(5, res.data.dealts.len());
}

#[test]
fn test_chart_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/chart_response_with_oddlot.json")).unwrap();
    let res: ChartResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("ODDLOT", res.data.info.typ);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!(87, res.data.chart.open.len());
    assert_eq!(87, res.data.chart.high.len());
    assert_eq!(87, res.data.chart.low.len());
    assert_eq!(87, res.data.chart.close.len());
    assert_eq!(87, res.data.chart.unix_timestamp.len());
}

#[test]
fn test_quote_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/quote_response_with_oddlot.json")).unwrap();
    let res: QuoteResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("ODDLOT", res.data.info.typ);
    assert_eq!(
        (-0.19f64).to_string(),
        res.data.quote.change_percent.to_string()
    );
    assert_eq!(0.19f64.to_string(), res.data.quote.amplitude.to_string());
    assert_eq!(5, res.data.quote.order.bids.len());
    assert_eq!(5, res.data.quote.order.asks.len());
    assert_eq!(
        26.45f64.to_string(),
        res.data.quote.price_high.price.to_string()
    );
    assert_eq!(
        26.4f64.to_string(),
        res.data.quote.price_low.price.to_string()
    );
    assert_eq!(
        26.45f64.to_string(),
        res.data.quote.price_open.price.to_string()
    );
    assert_eq!(
        26.43f64.to_string(),
        res.data.quote.price_avg.price.to_string()
    );
}

#[test]
fn test_meta_response_with_oddlot_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/meta_response_with_oddlot.json")).unwrap();
    let res: MetaResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("ODDLOT", res.data.info.typ);
    assert_eq!(
        26.5f64.to_string(),
        res.data.meta.price_reference.to_string()
    );
    assert_eq!(
        29.15f64.to_string(),
        res.data.meta.price_high_limit.to_string()
    );
    assert_eq!(
        23.85f64.to_string(),
        res.data.meta.price_low_limit.to_string()
    );
    assert!(!res.data.meta.can_day_buy_sell);
    assert_eq!("玉山金", res.data.meta.name_zh_tw);
}

#[test]
fn test_dealts_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/dealts_response_with_oddlot.json")).unwrap();
    let res: DealtsResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("ODDLOT", res.data.info.typ);
    assert_eq!(5, res.data.dealts.len());
}

#[test]
fn test_error_response_deserialize() {
    let input_json = r#"{
      "apiVersion": "0.3.0",
      "error": {
        "code": 401,
        "message": "Unauthorized"
      }
    }"#;
    let err: ErrorResponse = serde_json::from_str(input_json).unwrap();
    let got = FugleError::from(err);
    match got {
        FugleError::Unauthorized => {}
        _ => unreachable!(),
    }

    let input_json = r#"{
      "apiVersion": "0.3.0",
      "error": {
        "code": 403,
        "message": "RateLimitExceeded"
      }
    }"#;
    let err: ErrorResponse = serde_json::from_str(input_json).unwrap();
    let got = FugleError::from(err);
    match got {
        FugleError::RateLimitExceeded => {}
        _ => unreachable!(),
    }

    let input_json = r#"{
      "apiVersion": "0.3.0",
      "error": {
        "code": 404,
        "message": "ResourceNotFound"
      }
    }"#;
    let err: ErrorResponse = serde_json::from_str(input_json).unwrap();
    let got = FugleError::from(err);
    match got {
        FugleError::ResourceNotFound => {}
        _ => unreachable!(),
    }
}
