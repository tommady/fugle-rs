use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use futures_util::StreamExt;
use log::error;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::connect_async;

use crate::schema::Result;

pub(crate) struct Async {
    pub(crate) routine: Option<tokio::task::JoinHandle<()>>,
}

impl Async {
    pub(crate) async fn new<T>(
        uri: &str,
        sender: UnboundedSender<T>,
        done: Arc<AtomicBool>,
    ) -> Result<Async>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        let (mut socket, _) = connect_async(uri).await?;

        let routine = tokio::spawn(async move {
            while !done.load(Ordering::SeqCst) {
                if let Some(msg) = socket.next().await {
                    let m = match msg {
                        Ok(m) => match m.to_text() {
                            Ok(m) => match serde_json::from_str(m) {
                                Ok(m) => m,
                                Err(e) => {
                                    error!("{}", e);
                                    continue;
                                }
                            },
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
            }
            let _ = socket.close(None).await;
        });

        Ok(Async {
            routine: Some(routine),
        })
    }
}

impl super::Worker for Async {
    fn stop(&mut self) {
        if let Some(routine) = self.routine.take() {
            drop(routine)
        }
    }
}