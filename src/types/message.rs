use crate::types::{Contact, GroupInfo, ReactionData};

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    #[serde(rename = "chatInfo")]
    pub info: MessageInfo,
    #[serde(rename = "chatItem")]
    pub data: MessageData,
}

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum MessageInfo {
    Direct { contact: Box<Contact> },
    Group { group_info: Box<GroupInfo> },
}

#[derive(Debug, serde::Deserialize)]
pub struct MessageData {
    #[serde(rename = "chatDir")]
    pub direction: MessageDirection,
    pub meta: MessageMeta,
    pub content: MessageContent,
    /// Key: mentioned member's display name
    pub mentions: std::collections::HashMap<String, MessageMention>,
    pub reactions: Vec<MessageReaction>,
    pub file: Option<MessageFile>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MessageDirection {
    #[serde(rename = "directRcv")]
    DirectReceive,
    #[serde(rename = "directSnd")]
    DirectSend,
    #[serde(rename = "groupRcv")]
    GroupReceive,
    #[serde(rename = "groupSnd")]
    GroupSend,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMeta {
    #[serde(rename = "itemId")]
    pub id: usize,
    #[serde(rename = "itemTs")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "itemText")]
    pub text: String,
    #[serde(rename = "itemStatus")]
    pub status: MessageMetaStatus,
    #[serde(rename = "itemSharedMsgId")]
    pub shared_id: String,
    #[serde(rename = "itemEdited")]
    pub edited: bool,
    pub user_mention: bool,
    pub deletable: bool,
    pub editable: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MessageMetaStatus {
    #[serde(rename = "rcvNew")]
    ReceiveNew,
    #[serde(rename = "rcvRead")]
    ReceiveRead,
    #[serde(rename = "sndNew")]
    SendNew,
    #[serde(rename = "sndError")]
    SendError,
    #[serde(rename = "sndErrorAuth")]
    SendErrorAuth,
    #[serde(rename = "sndSent")]
    SendSent,
    #[serde(rename = "sndRcvd")]
    SendReceived,
}

#[derive(Debug, serde::Deserialize)]
pub struct MessageContent {
    pub r#type: MessageContentType,
    #[serde(rename = "msgContent")]
    pub content: MessageContentData,
}

#[derive(Debug, serde::Deserialize)]
pub enum MessageContentType {
    #[serde(rename = "rcvMsgContent")]
    SendMessageContent,
    #[serde(rename = "sndMsgContent")]
    RecieveMessageContent,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MessageContentData {
    Text {
        text: String,
    },
    Video {
        text: String,
        image: String,
        duration: usize,
    },
}

impl MessageContentData {
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::Text { text } | Self::Video { text, .. } => text,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMention {
    pub member_id: String,
    pub member_ref: MessageMentionRef,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMentionRef {
    pub group_member_id: usize,
    pub display_name: String,
    pub local_alias: String,
    pub member_role: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReaction {
    pub reaction: ReactionData,
    pub user_reacted: bool,
    pub total_reacted: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct MessageFile {
    #[serde(rename = "fileId")]
    pub id: usize,
    #[serde(rename = "fileName")]
    pub name: String,
    #[serde(rename = "fileSize")]
    pub size: usize,
    #[serde(rename = "fileStatus")]
    pub status: MessageFileStatus,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MessageFileStatus {
    #[serde(rename = "rcvInvitation")]
    ReceiveInvitation,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageFileProtocol {
    Xftp,
}
