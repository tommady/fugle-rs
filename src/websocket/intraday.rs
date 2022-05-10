use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};

use log::error;

#[cfg(feature = "websocket")]
use tungstenite::connect;

#[cfg(feature = "async-websocket")]
use futures_util::StreamExt;
#[cfg(feature = "async-websocket")]
use tokio;
#[cfg(feature = "async-websocket")]
use tokio_tungstenite::connect_async;

use crate::{
    errors::FugleError,
    intraday::{chart::ChartResponse, meta::MetaResponse, quote::QuoteResponse},
    schema::Result,
};

const INTRADAY_CHART: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/chart";
const INTRADAY_QUOTE: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/quote";
const INTRADAY_META: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/meta";

/// Accumulates options towards building an Intraday instance.
pub struct IntradayBuilder<'a> {
    token: &'a str,
    symbol_id: &'a str,
    is_odd_lot: bool,
}

impl<'a> Default for IntradayBuilder<'a> {
    fn default() -> IntradayBuilder<'a> {
        IntradayBuilder::new()
    }
}

impl<'a> IntradayBuilder<'a> {
    pub fn new() -> IntradayBuilder<'a> {
        IntradayBuilder {
            token: "demo",
            symbol_id: "",
            is_odd_lot: false,
        }
    }

    /// Setup your personal fugle token.
    ///
    /// By default the IntradayBuilder using fugle demo token which has limitations on listening,
    /// please login into below web page
    /// https://developer.fugle.tw/
    /// then find your personal token.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::websocket::IntradayBuilder;
    /// let ws = IntradayBuilder::new()
    ///     .token("b52153ae36747b17c8bdee801da19542")
    ///     .build();
    /// ```
    pub fn token(mut self, token: &'a str) -> IntradayBuilder {
        self.token = token;
        self
    }

    pub fn symbol_id(mut self, symbol_id: &'a str) -> IntradayBuilder {
        self.symbol_id = symbol_id;
        self
    }

    pub fn odd_lot(mut self) -> IntradayBuilder<'a> {
        self.is_odd_lot = true;
        self
    }

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
    /// # use fugle::websocket;
    ///
    /// let mut ws = websocket::IntradayBuilder::new().build();
    /// ```
    pub fn build(self) -> Intraday {
        Intraday {
            uri: format!(
                "symbolId={}&apiToken={}&oddLot={}",
                self.symbol_id, self.token, self.is_odd_lot,
            ),
            workers: vec![],
            #[cfg(feature = "async-websocket")]
            async_workers: vec![],
            done: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// Intraday is the Websocket listener to fugle wws endpoints.
pub struct Intraday {
    uri: String,
    workers: Vec<Worker>,
    #[cfg(feature = "async-websocket")]
    async_workers: Vec<AsyncWorker>,
    done: Arc<AtomicBool>,
}

impl Intraday {
    /// Listening fugle Chart endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # fn main() -> fugle::schema::Result<()> {
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let rx = ws.chart()?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn chart(&mut self) -> Result<Receiver<ChartResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_CHART, self.uri);
        match Worker::new(uri, tx, self.done.clone()) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn async_chart(&mut self) -> Result<Receiver<ChartResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_CHART, self.uri);
        match AsyncWorker::new(uri, tx, self.done.clone()).await {
            Ok(w) => {
                self.async_workers.push(w);
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
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let rx = ws.meta()?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn meta(&mut self) -> Result<Receiver<MetaResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_META, self.uri);
        match Worker::new(uri, tx, self.done.clone()) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn async_meta(&mut self) -> Result<Receiver<MetaResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_META, self.uri);
        match AsyncWorker::new(uri, tx, self.done.clone()).await {
            Ok(w) => {
                self.async_workers.push(w);
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
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let rx = ws.quote()?;
    /// let response = rx.recv()?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote(&mut self) -> Result<Receiver<QuoteResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_QUOTE, self.uri);
        match Worker::new(uri, tx, self.done.clone()) {
            Ok(w) => {
                self.workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn async_quote(&mut self) -> Result<Receiver<QuoteResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_QUOTE, self.uri);
        match AsyncWorker::new(uri, tx, self.done.clone()).await {
            Ok(w) => {
                self.async_workers.push(w);
                Ok(rx)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> Drop for Intraday {
    fn drop(&mut self) {
        self.done.store(true, Ordering::SeqCst);

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }

        if cfg!(feature = "async-websocket") {
            for worker in &mut self.async_workers {
                if let Some(routine) = worker.routine.take() {
                    drop(routine)
                }
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
                match socket.read_message() {
                    Ok(msg) => {
                        if let Err(e) = handler(sender.clone(), msg) {
                            error!("{}", e);
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
            let _ = socket.close(None);
        });

        Ok(Worker {
            thread: Some(thread),
        })
    }
}

struct AsyncWorker {
    routine: Option<tokio::task::JoinHandle<()>>,
}

impl AsyncWorker {
    async fn new<T>(uri: &str, sender: Sender<T>, done: Arc<AtomicBool>) -> Result<AsyncWorker>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        let (mut socket, _) = connect_async(uri).await?;

        let routine = tokio::spawn(async move {
            while !done.load(Ordering::SeqCst) {
                if let Some(msg) = socket.next().await {
                    if let Err(e) = handler(sender.clone(), msg.unwrap()) {
                        error!("{}", e);
                    }
                }
            }
            let _ = socket.close(None).await;
        });

        Ok(AsyncWorker {
            routine: Some(routine),
        })
    }
}

fn handler<T>(sender: Sender<T>, msg: tungstenite::Message) -> Result<()>
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
{
    let m = msg.to_text()?;
    if m.is_empty() {
        return Ok(());
    }

    sender
        .send(serde_json::from_str(m)?)
        .map_err(|_| FugleError::MpscSendError)
}
