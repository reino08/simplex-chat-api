mod builder;
pub use builder::ClientBuilder;

use std::{
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};

use futures::{
    SinkExt as _,
    channel::{mpsc::UnboundedSender, oneshot},
};

use crate::{
    command,
    stream::{Request, RequestError, Response},
};

#[cfg(target_arch = "wasm32")]
type SocketTx = dyn futures::Sink<Request, Error = RequestError>;
#[cfg(not(target_arch = "wasm32"))]
type SocketTx = dyn futures::Sink<Request, Error = RequestError> + Send;

/// A [`ClientBuilder`] which has been started with the [`ClientBuilder::run`] method
pub struct Client {
    counter: AtomicUsize,
    channel_tx: UnboundedSender<(String, oneshot::Sender<Response>)>,
    socket_tx: async_lock::Mutex<Pin<Box<SocketTx>>>,
}

impl Client {
    /// Sends an arbitary raw, unformatted command.
    /// Use a higher level wrapper instead.
    /// # Panics
    /// If the background task returned by [`ClientBuilder::run`] was dropped.
    pub async fn send_raw(&self, command: String) -> Result<Response, RequestError> {
        let (tx, rx) = oneshot::channel();
        let id = self.counter.fetch_add(1, Ordering::Relaxed).to_string();
        self.channel_tx
            .clone()
            .send((id.clone(), tx))
            .await
            .unwrap();
        self.socket_tx
            .lock()
            .await
            .send(Request { id, command })
            .await?;
        Ok(rx.await.unwrap())
    }

    pub async fn say(&self, message: command::CreateMessage) -> Result<Response, RequestError> {
        let dest = match message.recipient {
            command::Recipient::Direct(user) => format!("@'{user}'"),
            command::Recipient::Group(chat) => format!("#'{chat}'"),
        };

        self.send_raw(format!("{dest} {}", message.content)).await
    }

    pub async fn contacts(&self) -> Result<Response, RequestError> {
        self.send_raw("/contacts".to_owned()).await
    }
}
