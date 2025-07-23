use futures::{SinkExt as _, StreamExt as _};

use crate::stream::{Request, RequestError, Response, ResponseError};

/// Connects to a websocket at a given URL and manages it.
///
/// See also [`connect_ws`] for manual control over the WebSocket.
///
/// # Errors
/// If anything goes wrong establishing the WebSocket connection.
pub async fn connect(
    url: url::Url,
) -> Result<
    (
        impl futures::Sink<Request, Error = RequestError> + Unpin + 'static,
        impl futures::Stream<Item = Response> + Unpin + 'static,
    ),
    yawc::WebSocketError,
> {
    let socket = yawc::WebSocket::connect(url).await?;
    Ok(connect_ws(socket))
}

/// Wraps an existing WebSocket synchronously.
pub fn connect_ws(
    socket: yawc::WebSocket,
) -> (
    impl futures::Sink<Request, Error = RequestError> + Unpin + 'static,
    impl futures::Stream<Item = Response> + Unpin + 'static,
) {
    let (tx, rx) = socket.split();
    let tx = tx.with(|req: Request| std::future::ready(req.frame()));
    let rx = rx.filter_map(
        #[cfg(not(target_arch = "wasm32"))]
        |frame: yawc::FrameView| {
            std::future::ready(match frame.opcode {
                yawc::OpCode::Text => std::str::from_utf8(&frame.payload)
                    .map_err(ResponseError::UTF8Error)
                    .and_then(Response::parse)
                    .ok(),
                _ => None,
            })
        },
        #[cfg(target_arch = "wasm32")]
        |frame: Result<yawc::FrameView, yawc::WebSocketError>| {
            std::future::ready(match frame {
                Err(_) => None,
                Ok(frame) => std::str::from_utf8(&frame.payload)
                    .map_err(ResponseError::UTF8Error)
                    .and_then(Response::parse)
                    .ok(),
            })
        },
    );

    (tx, rx)
}
