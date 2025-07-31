use crate::types::{MessageData, MessageInfo};

#[derive(Debug, serde::Deserialize)]
pub struct Chat {
    #[serde(rename = "chatInfo")]
    pub info: MessageInfo,
    #[serde(rename = "chatItems")]
    pub items: Vec<MessageData>,
    #[serde(rename = "chatStats")]
    pub stats: ChatStats,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatStats {
    pub unread_count: usize,
    pub unread_mentions: usize,
    pub reports_count: usize,
    pub min_unread_item_id: usize,
    pub unread_chat: bool,
}
