mod builder;
pub use builder::ClientBuilder;

use std::sync::atomic::{AtomicUsize, Ordering};

use futures::{
    SinkExt as _,
    channel::{mpsc::UnboundedSender, oneshot},
};

use crate::{
    command,
    stream::{Request, RequestError, Response, ResponseData},
    types::{Contact, User, UserWithUnreadCount},
};

#[cfg(target_arch = "wasm32")]
type SocketTx = dyn futures::Sink<Request, Error = RequestError>;
#[cfg(not(target_arch = "wasm32"))]
type SocketTx = dyn futures::Sink<Request, Error = RequestError> + Send;

macro_rules! define_getter {
    ($name:ident, $ret: ty, $command:expr, $arm:pat => $res:expr $(, $(#[$meta:meta])*)?) => {
        pub async fn $name(&self) -> Result<$ret, ClientError> {
            let response = self.send_raw($command.to_owned()).await?;
            match response.data {
                $arm => $res,
                _ => Err(ClientError::UnexpectedResponse(response)),
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("an error occurred while making a request: {0}")]
    RequestError(#[from] RequestError),
    #[error("an unexpected output was received")]
    UnexpectedResponse(Response),
}

/// A [`ClientBuilder`] which has been started with the [`ClientBuilder::run`] method
pub struct Client {
    counter: AtomicUsize,
    channel_tx: UnboundedSender<(String, oneshot::Sender<Response>)>,
    socket_tx: async_lock::Mutex<std::pin::Pin<Box<SocketTx>>>,
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

    define_getter!(users, Vec<UserWithUnreadCount>, "/users", ResponseData::UsersList { users } => Ok(users),
        /// Returns all local profiles along with their amount of unread messages.
    );

    define_getter!(active_user, User, "/user", ResponseData::ActiveUser { user } => Ok(user),
        /// Returns the active profile.
    );

    define_getter!(contacts, Vec<Contact>, "/contacts", ResponseData::ContactsList { contacts, .. } => Ok(contacts));
}
