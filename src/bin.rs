use fugle::listener;
use pretty_env_logger;
use std::sync::mpsc;

pub fn main() {
    pretty_env_logger::init();

    let (tx, rx) = mpsc::channel();
    let mut lis = listener::Intraday::new("", tx.clone());
    if let Err(e) = lis.chart("2884") {
        panic!("{}", e);
    };

    loop {
        match rx.recv() {
            Ok(m) => println!("{:?}", m),
            Err(e) => panic!("{}", e),
        }
    }
    // let g = listener::intraday_chart("2884", "demo");
    // println!("{:?}", g);
}
