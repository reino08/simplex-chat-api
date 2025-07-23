use crate::types::{MessageData, MessageDirection, MessageInfo};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    #[serde(rename = "chatInfo")]
    pub info: MessageInfo,
    pub chat_reaction: ReactionInfo,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionInfo {
    pub chat_dir: MessageDirection,
    #[serde(rename = "chatItem")]
    pub message: MessageData,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub reaction: ReactionData,
}

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ReactionData {
    Emoji { emoji: String },
}
