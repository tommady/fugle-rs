use crate::schema::{ChartResponse, MetaResponse, QuoteResponse, Response, Result};
use log::error;
use serde_json;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};
use std::thread;
use tungstenite::connect;

const INTRADAY_CHART: &str = "wss://api.fugle.tw/realtime/v0/intraday/chart";
const INTRADAY_QUOTE: &str = "wss://api.fugle.tw/realtime/v0/intraday/quote";
const INTRADAY_META: &str = "wss://api.fugle.tw/realtime/v0/intraday/meta";

#[derive(Clone, Copy)]
enum Mode {
    Chart,
    Quote,
    Meta,
}

pub struct Intraday {
    token: &'static str,
    workers: Vec<Worker>,
    done: Arc<AtomicBool>,
    sender: Sender<Response>,
}

impl Intraday {
    pub fn new(token: &'static str, sender: Sender<Response>) -> Intraday {
        Intraday {
            token: token,
            workers: vec![],
            done: Arc::new(AtomicBool::new(false)),
            sender: sender,
        }
    }

    pub fn chart(&mut self, symbol_id: &str) -> Result<()> {
        match Worker::new(
            Mode::Chart,
            format!(
                "{}?symbolId={}&apiToken={}",
                INTRADAY_CHART, symbol_id, self.token
            ),
            self.sender.clone(),
            self.done.clone(),
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn meta(&mut self, symbol_id: &str) -> Result<()> {
        match Worker::new(
            Mode::Meta,
            format!(
                "{}?symbolId={}&apiToken={}",
                INTRADAY_META, symbol_id, self.token
            ),
            self.sender.clone(),
            self.done.clone(),
        ) {
            Ok(w) => {
                self.workers.push(w);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn quote(&mut self, symbol_id: &str) -> Result<()> {
        match Worker::new(
            Mode::Quote,
            format!(
                "{}?symbolId={}&apiToken={}",
                INTRADAY_QUOTE, symbol_id, self.token
            ),
            self.sender.clone(),
            self.done.clone(),
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
        mode: Mode,
        uri: String,
        sender: Sender<Response>,
        done: Arc<AtomicBool>,
    ) -> Result<Worker> {
        let (mut socket, _) = connect(uri)?;

        let thread = thread::spawn(move || {
            while !done.load(Ordering::SeqCst) {
                let mut socket_receiver = || -> tungstenite::Result<String> {
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

                let m = txt.as_str();
                let sending = || -> Result<()> {
                    match mode {
                        Mode::Chart => {
                            let c: ChartResponse = serde_json::from_str(m)?;
                            sender.send(Response::ChartResponse(c))?;
                        }
                        Mode::Quote => {
                            let q: QuoteResponse = serde_json::from_str(m)?;
                            sender.send(Response::QuoteResponse(q))?;
                        }
                        Mode::Meta => {
                            let m: MetaResponse = serde_json::from_str(m)?;
                            sender.send(Response::MetaResponse(m))?;
                        }
                    }
                    Ok(())
                };

                if let Err(e) = sending() {
                    error!("sending error:{}", e);
                }
            }
        });

        Ok(Worker {
            thread: Some(thread),
        })
    }
}
