use fugle::schema::*;
use serde_json;
use std::fs::File;
use std::path::Path;

#[test]
fn test_chart_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/chart_response.json")).unwrap();
    let res: ChartResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    // assert_eq!("TSE", res.data.info.market);
    assert_ne!(0, res.data.chart.len());
}

#[test]
fn test_quote_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/quote_response.json")).unwrap();
    let res: QuoteResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    // assert_eq!("TSE", res.data.info.market);
    assert_eq!(0.00792079207921, res.data.quote.change_percent);
    assert_eq!(0.0138613861386, res.data.quote.amplitude);
    assert_ne!(0, res.data.quote.order.best_bids.len());
    assert_ne!(0, res.data.quote.order.best_asks.len());
    assert_eq!(25.6, res.data.quote.price_high.price);
    assert_eq!(25.25, res.data.quote.price_low.price);
    assert_eq!(25.35, res.data.quote.price_open.price);
}

#[test]
fn test_meta_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/meta_response.json")).unwrap();
    let res: MetaResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("EQUITY", res.data.info.typ);
    assert_eq!(26.5, res.data.meta.price_reference);
    assert_eq!(29.15, res.data.meta.price_high_limit);
    assert_eq!(23.85, res.data.meta.price_low_limit);
    assert_eq!(true, res.data.meta.can_day_buy_sell);
    assert_eq!("玉山金", res.data.meta.name_zh_tw);
    assert_eq!("金融保險", res.data.meta.industry_zh_tw);
}

#[test]
fn test_dealts_response_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/dealts_response.json")).unwrap();
    let res: DealtsResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    // assert_eq!("TSE", res.data.info.market);
    assert_ne!(0, res.data.dealts.len());
}

#[test]
fn test_chart_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/chart_response_with_oddlot.json")).unwrap();
    let res: ChartResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    // assert_eq!("TSE", res.data.info.market);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_ne!(0, res.data.chart.len());
}

#[test]
fn test_quote_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/quote_response_with_oddlot.json")).unwrap();
    let res: QuoteResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    // assert_eq!("TSE", res.data.info.market);
    assert_eq!(0.00990099009901, res.data.quote.change_percent);
    assert_eq!(0.00792079207921, res.data.quote.amplitude);
    assert_ne!(0, res.data.quote.order.best_bids.len());
    assert_ne!(0, res.data.quote.order.best_asks.len());
    assert_eq!(25.6, res.data.quote.price_high.price);
    assert_eq!(25.4, res.data.quote.price_low.price);
    assert_eq!(25.55, res.data.quote.price_open.price);
}

#[test]
fn test_meta_response_with_oddlot_deserialize() {
    let json_file = File::open(Path::new("tests/testdata/meta_response_with_oddlot.json")).unwrap();
    let res: MetaResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.3.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    assert_eq!("TSE", res.data.info.market);
    assert_eq!("ODDLOT", res.data.info.typ);
    assert_eq!(26.5, res.data.meta.price_reference);
    assert_eq!(29.15, res.data.meta.price_high_limit);
    assert_eq!(23.85, res.data.meta.price_low_limit);
    assert_eq!(false, res.data.meta.can_day_buy_sell);
    assert_eq!("玉山金", res.data.meta.name_zh_tw);
}

#[test]
fn test_dealts_response_with_oddlot_deserialize() {
    let json_file =
        File::open(Path::new("tests/testdata/dealts_response_with_oddlot.json")).unwrap();
    let res: DealtsResponse = serde_json::from_reader(json_file).unwrap();

    assert_eq!("0.2.0", res.api_version);
    assert_eq!("2884", res.data.info.symbol_id);
    // assert_eq!("TSE", res.data.info.market);
    assert_ne!(0, res.data.dealts.len());
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
        FugleError::Unauthorized => assert!(true),
        _ => assert!(false),
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
        FugleError::RateLimitExceeded => assert!(true),
        _ => assert!(false),
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
        FugleError::ResourceNotFound => assert!(true),
        _ => assert!(false),
    }
}
