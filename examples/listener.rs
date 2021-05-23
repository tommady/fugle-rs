use fugle::listener;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut lis = listener::Intraday::new("demo", tx.clone());

    let works = vec![
        lis.chart("2884", true),
        lis.meta("2884", false),
        lis.quote("2884", true),
    ];
    for work in &works {
        if let Err(e) = work {
            panic!("{}", e);
        }
    }

    for _ in 0..works.len() {
        match rx.recv() {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{}", e),
        }
    }
}
