#[cfg(feature = "websocket")]
mod block;
#[cfg(feature = "websocket")]
use block::Block as BlockWorker;

#[cfg(feature = "async-websocket")]
mod r#async;
#[cfg(feature = "async-websocket")]
use r#async::Async as AsyncWorker;

#[cfg(feature = "websocket")]
use std::sync::mpsc::{channel, Receiver};
#[cfg(feature = "async-websocket")]
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::schema::{ChartResponse, MetaResponse, QuoteResponse, Result};

const INTRADAY_CHART: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/chart";
const INTRADAY_QUOTE: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/quote";
const INTRADAY_META: &str = "wss://api.fugle.tw/realtime/v0.3/intraday/meta";

/// Accumulates options towards building an Intraday instance of WebSocket.
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
    /// Returns a default IntradayBuilder with
    /// * fugle "demo" token
    /// * empty symbol id
    /// * false of odd lot
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

    /// Set a stock identification code to receive WebSocket data.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::websocket::IntradayBuilder;
    /// let ws = IntradayBuilder::new()
    ///     .symbol_id("b52153ae36747b17c8bdee801da19542")
    ///     .build();
    /// ```
    pub fn symbol_id(mut self, symbol_id: &'a str) -> IntradayBuilder {
        self.symbol_id = symbol_id;
        self
    }

    /// To see odd lotter or not.
    ///
    /// # Example:
    ///
    /// ```
    /// # use fugle::websocket::IntradayBuilder;
    /// let ws = IntradayBuilder::new()
    ///     .odd_lot()
    ///     .build();
    /// ```
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
            done: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// Intraday is the Websocket listener to fugle wws endpoints.
pub struct Intraday {
    uri: String,
    workers: Vec<Box<dyn Worker>>,
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
    #[cfg(feature = "websocket")]
    pub fn chart(&mut self) -> Result<Receiver<ChartResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_CHART, self.uri);
        let worker = BlockWorker::new(uri, tx, self.done.clone())?;
        self.workers.push(Box::new(worker));
        Ok(rx)
    }

    /// Listening fugle Chart endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> fugle::schema::Result<()> {
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let mut rx = ws.async_chart().await?;
    /// let response = rx.recv().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async-websocket")]
    pub async fn async_chart(&mut self) -> Result<UnboundedReceiver<ChartResponse>> {
        let (tx, rx) = unbounded_channel();
        let uri = &format!("{}?{}", INTRADAY_CHART, self.uri);
        let worker = AsyncWorker::new(uri, tx, self.done.clone()).await?;
        self.workers.push(Box::new(worker));
        Ok(rx)
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
    #[cfg(feature = "websocket")]
    pub fn meta(&mut self) -> Result<Receiver<MetaResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_META, self.uri);
        let worker = BlockWorker::new(uri, tx, self.done.clone())?;
        self.workers.push(Box::new(worker));
        Ok(rx)
    }

    /// Listening fugle Meta endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> fugle::schema::Result<()> {
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let mut rx = ws.async_meta().await?;
    /// let response = rx.recv().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async-websocket")]
    pub async fn async_meta(&mut self) -> Result<UnboundedReceiver<MetaResponse>> {
        let (tx, rx) = unbounded_channel();
        let uri = &format!("{}?{}", INTRADAY_META, self.uri);
        let worker = AsyncWorker::new(uri, tx, self.done.clone()).await?;
        self.workers.push(Box::new(worker));
        Ok(rx)
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
    #[cfg(feature = "websocket")]
    pub fn quote(&mut self) -> Result<Receiver<QuoteResponse>> {
        let (tx, rx) = channel();
        let uri = &format!("{}?{}", INTRADAY_QUOTE, self.uri);
        let worker = BlockWorker::new(uri, tx, self.done.clone())?;
        self.workers.push(Box::new(worker));
        Ok(rx)
    }

    /// Listening fugle Quote endpoint.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> fugle::schema::Result<()> {
    /// # use fugle::websocket::IntradayBuilder;
    ///
    /// let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
    ///
    /// let mut rx = ws.async_quote().await?;
    /// let response = rx.recv().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async-websocket")]
    pub async fn async_quote(&mut self) -> Result<UnboundedReceiver<QuoteResponse>> {
        let (tx, rx) = unbounded_channel();
        let uri = &format!("{}?{}", INTRADAY_QUOTE, self.uri);
        let worker = AsyncWorker::new(uri, tx, self.done.clone()).await?;
        self.workers.push(Box::new(worker));
        Ok(rx)
    }
}

impl Drop for Intraday {
    fn drop(&mut self) {
        self.done.store(true, Ordering::SeqCst);

        for worker in self.workers.iter_mut() {
            worker.stop();
        }
    }
}

pub(crate) trait Worker: Send {
    fn stop(&mut self);
}
