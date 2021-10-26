use crate::schema::{FugleError, Response, ResponseType, Result};
use log::error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};
use std::thread;
use tungstenite::connect;

const INTRADAY_CHART: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/chart";
const INTRADAY_QUOTE: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/quote";
const INTRADAY_META: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/meta";

/// Intraday is the Websocket listener to fugle wws endpoints.
pub struct Intraday {
    token: String,
    workers: Vec<Worker>,
    done: Arc<AtomicBool>,
    sender: Sender<Response>,
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
    /// # use fugle::listener;
    /// # use std::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel();
    /// let mut lis = listener::Intraday::new("demo", tx.clone());
    /// ```
    pub fn new(token: &str, sender: Sender<Response>) -> Intraday {
        Intraday {
            token: token.to_owned(),
            workers: vec![],
            done: Arc::new(AtomicBool::new(false)),
            sender,
        }
    }

    /// Listening fugle Chart endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::listener;
    /// # use std::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel();
    /// let mut lis = listener::Intraday::new("demo", tx.clone());
    ///
    /// lis.chart("2884", true);
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn chart(&mut self, symbol_id: &str, odd_lot: bool) -> Result<()> {
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_CHART, symbol_id, self.token, odd_lot,
            ),
            self.sender.clone(),
            self.done.clone(),
            ResponseType::Chart,
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(())
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
    /// # use fugle::listener;
    /// # use std::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel();
    /// let mut lis = listener::Intraday::new("demo", tx.clone());
    ///
    /// lis.meta("2884", true);
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn meta(&mut self, symbol_id: &str, odd_lot: bool) -> Result<()> {
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_META, symbol_id, self.token, odd_lot,
            ),
            self.sender.clone(),
            self.done.clone(),
            ResponseType::Meta,
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(())
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
    /// # use fugle::listener;
    /// # use std::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel();
    /// let mut lis = listener::Intraday::new("demo", tx.clone());
    ///
    /// lis.quote("2884", true);
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote(&mut self, symbol_id: &str, odd_lot: bool) -> Result<()> {
        match Worker::new(
            &format!(
                "{}?symbolId={}&apiToken={}&oddLot={}",
                INTRADAY_QUOTE, symbol_id, self.token, odd_lot,
            ),
            self.sender.clone(),
            self.done.clone(),
            ResponseType::Quote,
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(())
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
    fn new(
        uri: &str,
        sender: Sender<Response>,
        done: Arc<AtomicBool>,
        resposne_type: ResponseType,
    ) -> Result<Worker> {
        let (mut socket, _) = connect(uri)?;

        let thread = thread::spawn(move || {
            while !done.load(Ordering::SeqCst) {
                let mut socket_receiver = || -> Result<String> {
                    let msg = socket.read_message()?;
                    let txt = msg.to_text()?;
                    Ok(txt.to_owned())
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
                    match resposne_type {
                        ResponseType::Chart => sender
                            .send(Response::Chart(serde_json::from_str(txt.as_str())?))
                            .map_err(|_| FugleError::MpscSendError)?,
                        ResponseType::Meta => sender
                            .send(Response::Meta(serde_json::from_str(txt.as_str())?))
                            .map_err(|_| FugleError::MpscSendError)?,
                        ResponseType::Quote => sender
                            .send(Response::Quote(serde_json::from_str(txt.as_str())?))
                            .map_err(|_| FugleError::MpscSendError)?,
                        _ => unreachable!("not supported response type"),
                    }
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
