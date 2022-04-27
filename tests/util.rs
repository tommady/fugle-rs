#![macro_use]
use std::{sync::mpsc, thread, time::Duration};

use fugle::{errors::FugleError, schema::Result};

macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected: {} but got: {:?}", stringify!($($pattern)+), e),
        }
    }
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
fn test_assert_err() {
    let some_fn = || -> Result<()> { Err(FugleError::ResourceNotFound) };
    assert_err!(some_fn(), Err(FugleError::ResourceNotFound));
}

#[test]
#[should_panic]
fn test_timeout_after() {
    timeout_after(Duration::from_millis(100), || {
        thread::sleep(Duration::from_millis(200));
    })
}
