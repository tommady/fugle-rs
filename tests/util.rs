#![macro_use]
use fugle::crawler;
use fugle::schema::Response;
use std::{sync::mpsc, thread, time::Duration};

macro_rules! fetch_enum {
    ($enum:path, $expr:expr) => {{
        if let $enum(item) = $expr {
            item
        } else {
            panic!("failed to extract enum:{:?}", $expr)
        }
    }};
}

pub(crate) fn timeout_after<T, F>(d: Duration, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    let (done_tx, done_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = f();
        done_tx.send(()).expect("unable to send completion signal");
        val
    });

    match done_rx.recv_timeout(d) {
        Ok(_) => handle.join().expect("thread panicked"),
        Err(_) => panic!("thread took too long"),
    }
}

#[test]
fn test_fetch_enum() {
    let res = crawler::IntradayBuilder::new()
        .build()
        .chart("2884")
        .call()
        .unwrap();
    let v = fetch_enum!(Response::Chart, res);
    assert_eq!(v.data.info.symbol_id, "2884");
}

#[test]
#[should_panic]
fn test_timeout_after() {
    timeout_after(Duration::from_millis(100), || {
        thread::sleep(Duration::from_millis(200));
    })
}
