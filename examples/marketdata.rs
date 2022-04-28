use fugle::{
    errors::FugleError,
    marketdata::{candles::CandlesResponse, MarketdataBuilder},
};

fn main() {
    let agent = MarketdataBuilder::default().build();

    match agent.candles("2884").call() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match agent
        .candles("2884")
        .from("2022-04-21")
        .to("2022-04-28")
        .call()
    {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    // retry on 403 error.
    // there are many retry libraries better than this,
    // here is just a simple example.
    let mut result = CandlesResponse::default();

    'retry_loop: for _ in 0..3 {
        match agent.candles("2884").call() {
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
