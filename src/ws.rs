use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};

use log::error;
use tungstenite::connect;

use crate::{
    errors::FugleError,
    intraday::{chart::ChartResponse, meta::MetaResponse, quote::QuoteResponse},
    schema::Result,
};

const INTRADAY_CHART: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/chart";
const INTRADAY_QUOTE: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/quote";
const INTRADAY_META: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/meta";

/// Intraday is the Websocket listener to fugle wws endpoints.
pub struct Intraday {
    token: &'static str,
    workers: Vec<Worker>,
    done: Arc<AtomicBool>,
}

impl Intraday {
    /// Returns an Intraday instance.
    ///
    /// When listening on each endpoint,
    /// Intraday will fork a thread to do the listening job,
    /// so need to use mpsc::channel receiver to receive response data.
    ///
    /// And as a daemon like process, it won't break while any error ocurs,
    /// instead it will log the error.
    ///
    /// Please reference to below link to know how to print the log out.
    /// https://github.com/rust-lang/log
    ///
    /// Example:
    ///
    /// ```
    /// # use fugle::ws;
    ///
    /// let mut lis = ws::Intraday::new("demo");
    /// ```
    pub fn new(token: &'static str) -> Intraday {
        Intraday {
            token,
            workers: vec![],
            done: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Listening fugle Chart endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::ws;
    ///
    /// let mut lis = ws::Intraday::new("demo");
    ///
    /// let rx = lis.chart("2884", true)?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn chart(&mut self, symbol_id: &str, odd_lot: bool) -> Result<Receiver<ChartResponse>> {
        let (tx, rx) = channel();
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_CHART, symbol_id, self.token, odd_lot,
            ),
            tx,
            self.done.clone(),
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }

    /// Listening fugle Meta endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::ws;
    ///
    /// let mut lis = ws::Intraday::new("demo");
    ///
    /// let rx = lis.meta("2884", true)?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn meta(&mut self, symbol_id: &str, odd_lot: bool) -> Result<Receiver<MetaResponse>> {
        let (tx, rx) = channel();
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_META, symbol_id, self.token, odd_lot,
            ),
            tx,
            self.done.clone(),
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }

    /// Listening fugle Quote endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::ws;
    ///
    /// let mut lis = ws::Intraday::new("demo");
    ///
    /// let rx = lis.quote("2884", true)?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote(&mut self, symbol_id: &str, odd_lot: bool) -> Result<Receiver<QuoteResponse>> {
        let (tx, rx) = channel();
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_QUOTE, symbol_id, self.token, odd_lot,
            ),
            tx,
            self.done.clone(),
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for Intraday {
    fn drop(&mut self) {
        self.done.store(true, Ordering::SeqCst);
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new<T>(uri: &str, sender: Sender<T>, done: Arc<AtomicBool>) -> Result<Worker>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        let (mut socket, _) = connect(uri)?;

        let thread = thread::spawn(move || {
            while !done.load(Ordering::SeqCst) {
                let mut socket_receiver = || -> Result<String> {
                    let msg = socket.read_message()?;
                    Ok(msg.to_text()?.to_string())
                };

                let txt = match socket_receiver() {
                    Ok(v) => v,
                    Err(e) => {
                        error!("socket_receiver error:{}", e);
                        continue;
                    }
                };

                if txt.is_empty() {
                    continue;
                }

                let sending = || -> Result<()> {
                    sender
                        .send(serde_json::from_str(txt.as_str())?)
                        .map_err(|_| FugleError::MpscSendError)?;
                    Ok(())
                };

                if let Err(e) = sending() {
                    error!("sending error:{}", e);
                }
            }
            let _ = socket.close(None);
        });

        Ok(Worker {
            thread: Some(thread),
        })
    }
}
