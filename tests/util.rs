#![macro_use]
use std::time::Duration;

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
    use std::{sync::mpsc, thread};

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
pub(crate) async fn async_timeout_after<F, Fut>(d: Duration, f: F)
where
    F: FnOnce() -> Fut,
    F: Send + 'static,
    Fut: std::future::Future,
    Fut: Send + 'static,
{
    use tokio::{
        sync::oneshot::channel,
        time::{interval_at, Instant},
    };

    let mut timeout = interval_at(Instant::now() + d, d);
    let (done_tx, done_rx) = channel();
    tokio::spawn(async {
        f().await;
        done_tx.send(()).expect("unable to send completion signal");
    });

    'looper: loop {
        tokio::select! {
            _ = done_rx => { break 'looper }
            _ = timeout.tick() => { panic!("routine took too long") }
        }
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
        std::thread::sleep(Duration::from_millis(200));
    })
}

#[tokio::test]
#[should_panic]
async fn test_async_timeout_after() {
    async_timeout_after(Duration::from_millis(100), move || async move {
        tokio::time::sleep(Duration::from_millis(200)).await;
    })
    .await
}
