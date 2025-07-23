#[derive(Debug, serde::Serialize)]
pub struct Request {
    #[serde(rename = "corrId")]
    pub id: String,
    #[serde(rename = "cmd")]
    pub command: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("websocket error: {0}")]
    WebSocketError(#[from] yawc::WebSocketError),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl Request {
    pub(crate) fn frame(self) -> Result<yawc::FrameView, RequestError> {
        Ok(yawc::FrameView::text(serde_json::to_string(&self)?))
    }
}
