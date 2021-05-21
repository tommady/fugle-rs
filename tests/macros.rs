#![macro_use]
macro_rules! fetch_enum {
    ($enum:path, $expr:expr) => {{
        if let $enum(item) = $expr {
            item
        } else {
            panic!("failed to extract enum:{:?}", $expr)
        }
    }};
}

// use fugle::crawler;
// use fugle::schema::{Data, DealtsData, Info, Response};
// #[test]
// fn test_fetch_enum() {
//     let res = crawler::IntradayBuilder::new()
//         .build()
//         .chart("2884")
//         .call()
//         .unwrap();
//     // let response = Response {
//     //     api_version: "testing".to_string(),
//     //     data: Data::DealtsData(DealtsData {
//     //         info: Info::default(),
//     //         dealts: vec![],
//     //     }),
//     // };
//     let v = fetch_enum!(Data::DealtsData, res.data);
//     assert_eq!(v.info.symbol_id, "0");
// }
