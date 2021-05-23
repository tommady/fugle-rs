use fugle::crawler;

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
}
