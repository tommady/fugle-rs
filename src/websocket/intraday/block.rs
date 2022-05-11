use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
};

use log::error;
use tungstenite::connect;

use crate::schema::Result;

pub(crate) struct Block {
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Block {
    pub(crate) fn new<T>(uri: &str, sender: Sender<T>, done: Arc<AtomicBool>) -> Result<Block>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        let (mut socket, _) = connect(uri)?;

        let thread = thread::spawn(move || {
            while !done.load(Ordering::SeqCst) {
                match socket.read_message() {
                    Ok(msg) => {
                        let m = match msg.to_text() {
                            Ok(m) => match serde_json::from_str(m) {
                                Ok(j) => j,
                                Err(e) => {
                                    error!("{}", e);
                                    continue;
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                continue;
                            }
                        };
                        if let Err(e) = sender.send(m) {
                            error!("{}", e);
                            continue;
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
            let _ = socket.close(None);
        });

        Ok(Block {
            thread: Some(thread),
        })
    }
}

impl super::Worker for Block {
    fn stop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
