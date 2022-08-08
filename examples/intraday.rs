use fugle::{
    errors::FugleError,
    http::{
        intraday::{ChartRequest, DealtsRequest, MetaRequest, QuoteRequest, VolumesRequest},
        RestfulBuilder,
    },
    schema::MetaResponse,
};

fn main() {
    let client = RestfulBuilder::new().build().unwrap();

    match client.call(ChartRequest::new().symbol_id("2884")) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match client.call(MetaRequest::new().symbol_id("2884").odd_lot(false)) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match client.call(QuoteRequest::new().symbol_id("2884").odd_lot(true)) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match client.call(DealtsRequest::new().symbol_id("2884").limit(10)) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match client.call(VolumesRequest::new().symbol_id("2884")) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    // retry on 403 error.
    // there are many retry libraries better than this,
    // here is just a simple example.
    let mut result = MetaResponse::default();

    'retry_loop: for _ in 0..3 {
        match client.call(MetaRequest::default().symbol_id("2884")) {
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
