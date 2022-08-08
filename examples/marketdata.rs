use fugle::{
    errors::FugleError,
    http::{marketdata::CandlesRequest, RestfulBuilder},
    schema::CandlesResponse,
};

fn main() {
    let client = RestfulBuilder::default().build().unwrap();

    // match client.candles("2884").call() {
    match client.call(CandlesRequest::new().symbol_id("2884")) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match client.call(
        CandlesRequest::new()
            .symbol_id("2884")
            .from("2022-08-01")
            .to("2022-08-08"),
    ) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    // retry on 403 error.
    // there are many retry libraries better than this,
    // here is just a simple example.
    let mut result = CandlesResponse::default();

    'retry_loop: for _ in 0..3 {
        match client.call(CandlesRequest::default().symbol_id("2884")) {
            Ok(v) => {
                result = v;
                break 'retry_loop;
            }
            Err(e) => match e {
                FugleError::RateLimitExceeded => {
                    // sleep a second.
                    // based on fugle document,
                    // https://github.com/fortuna-intelligence/fugle-realtime-docs
                    // every min allows 60 requests.
                    continue 'retry_loop;
                }
                _ => {
                    // other errors
                    println!("{}", e);
                    break 'retry_loop;
                }
            },
        };
    }
    println!("{:?}", result);
}
