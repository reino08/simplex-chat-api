use std::{pin::Pin, sync::atomic::AtomicUsize};

use futures::{
    Sink, SinkExt, Stream, StreamExt as _,
    channel::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};

use crate::stream::{Request, RequestError, Response};

macro_rules! types {
    ($($send:tt)*) => {
        type SocketTx = dyn Sink<Request, Error = RequestError> + Unpin $($send)* + 'static;
        type SocketRx = dyn Stream<Item = Response> + Unpin $($send)* + 'static;
        type BackgroundTask = Pin<Box<dyn Future<Output = ()> + 'static $($send)*>>;
    };
}

#[cfg(target_arch = "wasm32")]
types!();
#[cfg(not(target_arch = "wasm32"))]
types!(+ Send);

pub struct ClientBuilder {
    socket_tx: Option<Pin<Box<SocketTx>>>,
    socket_rx: Box<SocketRx>,
}

impl ClientBuilder {
    /// See also [`Self::new_ws`] for manual control over the WebSocket.
    ///
    /// # Errors
    /// If anything goes wrong establishing the WebSocket connection.
    pub async fn new(url: url::Url) -> Result<Self, yawc::WebSocketError> {
        let (tx, rx) = crate::stream::connect(url).await?;
        Ok(Self {
            socket_tx: Some(Box::pin(tx)),
            socket_rx: Box::new(rx),
        })
    }

    #[must_use]
    pub fn new_ws(socket: yawc::WebSocket) -> Self {
        let (tx, rx) = crate::stream::connect_ws(socket);
        Self {
            socket_tx: Some(Box::pin(tx)),
            socket_rx: Box::new(rx),
        }
    }

    #[allow(clippy::missing_panics_doc)] // cannot panic
    #[must_use]
    pub fn run(mut self) -> (super::Client, UnboundedReceiver<Response>, BackgroundTask) {
        let (channel_tx, channel_rx) = mpsc::unbounded();
        let (event_tx, event_rx) = mpsc::unbounded();

        (
            super::Client {
                counter: AtomicUsize::new(0),
                channel_tx,
                socket_tx: async_lock::Mutex::new(self.socket_tx.take().unwrap()),
            },
            event_rx,
            Box::pin(self.background(channel_rx, event_tx)),
        )
    }

    async fn background(
        self,
        channel_rx: UnboundedReceiver<(String, oneshot::Sender<Response>)>,
        mut event_tx: UnboundedSender<Response>,
    ) {
        let mut socket_rx = Box::pin(self.socket_rx).fuse();
        let mut channel_rx = channel_rx.fuse();
        let mut collection = std::collections::HashMap::new();

        loop {
            futures::select_biased! {
                item = channel_rx.next() => {
                    let Some((id, channel)) = item else { return; };
                    if collection.insert(id, channel).is_some() {
                        log::error!("Tried to insert duplicate id, a request will now be left hanging indefinitely");
                    }
                },
                item = socket_rx.next() => {
                    let Some(response) = item else { return; };
                    match &response.id {
                        None => drop(event_tx.send(response).await),
                        Some(id) => {
                            let Some(channel) = collection.remove(id) else {
                                log::warn!("Unmatched id {id}, a request has likely been dropped");
                                continue;
                            };

                            drop(channel.send(response));
                        }
                    }
                },
            }
        }
    }
}
