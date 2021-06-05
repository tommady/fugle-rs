use fugle::crawler;
use fugle::schema;

fn main() {
    let agent = crawler::IntradayBuilder::new().build();

    match agent.chart("2884").call() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match agent.meta("2884").odd_lot(false).call() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match agent.quote("2884").odd_lot(true).call() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match agent.dealts("2884").limit(10).call() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    // retry on 403 error.
    // there are many retry libraries better than this,
    // here is just a simple example.
    let mut result = schema::MetaResponse::default();

    'retry_loop: for _ in 0..3 {
        match agent.meta("2884").call() {
            Ok(v) => {
                if let schema::Response::Meta(meta) = v {
                    result = meta;
                }
                break 'retry_loop;
            }
            Err(e) => match e {
                schema::FugleError::RateLimitExceeded => {
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
