use fugle::crawler;

fn main() {
    match crawler::intraday_chart("2884", "demo") {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match crawler::intraday_meta("2884", "demo") {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match crawler::intraday_quote("2884", "demo") {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    match crawler::intraday_dealts("2884", "demo", 0, 0) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
}
