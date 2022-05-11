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
                if let Some(Ok(msg)) = socket.next().await {
                    if let Ok(m) = msg.to_text() {
                        if let Ok(m) = serde_json::from_str(m) {
                            if let Err(e) = sender.send(m) {
                                error!("{}", e);
                            }
                        }
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

#[cfg(test)]
mod test {
    use tokio::sync::mpsc::unbounded_channel;

    use super::{
        super::{QuoteResponse, Worker, INTRADAY_QUOTE},
        *,
    };

    #[tokio::test]
    async fn test_async_worker_stop() {
        let (tx, _) = unbounded_channel::<QuoteResponse>();
        let done = Arc::new(AtomicBool::new(false));
        let mut worker = Async::new(
            &format!("{}?symbolId=2884&apiToken=demo", INTRADAY_QUOTE),
            tx,
            done.clone(),
        )
        .await
        .unwrap();

        done.store(true, Ordering::SeqCst);
        worker.stop();
    }
}
