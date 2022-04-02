use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use eyre::{Result, WrapErr};
use tokio::sync;
use tokio::task;

use crate::data::Data;

pub struct Client {
    socket_task: JoinHandle<Result<()>>,
    shutdown_signal: sync::oneshot::Sender<()>,
    data: sync::mpsc::Receiver<Data>,
}
impl Client {
    pub async fn new<C>(connect_to: C) -> Result<Self>
    where
        C: IntoClientRequest + Clone + Unpin + Send + Sync + 'static,
    {
        let (shutdown_signal, shutdown_signal_recv) = sync::oneshot::channel();
        let (data_send, data_recv) = sync::mpsc::channel(1);

        let socket_task = task::spawn(async move {
            let mut shutdown_signal = shutdown_signal_recv;
            loop {
                tokio::select! {
                    _ = &mut shutdown_signal => {
                        break;
                    }
                    else => {
                        let (socket, _) = connect_async(connect_to.clone())
                            .await.wrap_err("Could not open websocket connection")?;
                            todo!("Create Framed, stream into `data_send`")
                    }
                }
            }
            Ok(())
        });

        Ok(Self {
            socket_task,
            shutdown_signal,
            data: data_recv,
        })
    }

    pub async fn shutdown(self) -> Result<()> {
        drop(self.shutdown_signal);
        self.socket_task.await.wrap_err("Failed to join!")?
    }

    pub async fn join(self) -> Result<()> {
        self.socket_task.await.wrap_err("Failed to join!")?
    }
}
