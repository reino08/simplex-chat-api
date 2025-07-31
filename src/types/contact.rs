use crate::types::{MergedPreferences, Profile};

/// Represents an external user
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub contact_id: usize,
    pub local_display_name: String,
    pub profile: Profile,
    #[serde(rename = "activeConn")]
    pub active_connection: Option<Connection>,
    pub contact_used: bool,
    pub contact_status: String,
    pub chat_settings: ChatSettings,
    /// This field is probably empty. Use [`merged_preferences`] instead.
    pub user_preferences: std::collections::HashMap<String, serde_json::Value>,
    pub merged_preferences: MergedPreferences,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "chatTs")]
    pub chat_timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "contactGrpInvSent")]
    pub contact_group_invite_sent: bool,
    pub chat_tags: Vec<serde_json::Value>,
    pub chat_deleted: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    #[serde(rename = "connId")]
    pub id: usize,
    #[serde(rename = "agentConnId")]
    pub agent_connection_id: String,
    #[serde(rename = "connChatVersion")]
    pub chat_version: usize,
    #[serde(rename = "peerChatVRange")]
    pub peer_chat_version_range: VersionRange,
    #[serde(rename = "connLevel")]
    pub level: usize,
    pub via_user_contact_link: Option<usize>,
    pub via_group_link: bool,
    pub group_link_id: Option<String>,
    pub x_contact_id: Option<String>,
    #[serde(rename = "connType")]
    pub r#type: ConnectionType,
    #[serde(rename = "connStatus")]
    pub status: String,
    #[serde(rename = "contactConnInitiated")]
    pub connection_initiated: bool,
    pub local_alias: String,
    pub entity_id: usize,
    pub pq_support: bool,
    pub pq_encryption: bool,
    #[serde(rename = "pqSndEnabled")]
    pub pq_send_enabled: Option<bool>,
    #[serde(rename = "pqRcvEnabled")]
    pub pq_receive_enabled: Option<bool>,
    pub auth_err_counter: usize,
    pub quota_err_counter: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionType {
    Contact,
    Member,
}

#[derive(Debug, serde::Deserialize)]
pub struct VersionRange {
    #[serde(rename = "minVersion")]
    pub minimum_version: usize,
    #[serde(rename = "maxVersion")]
    pub maximum_version: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatSettings {
    #[serde(rename = "enableNtfs")]
    pub enable_notifications: String,
    pub favorite: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct ContactLink {
    #[serde(rename = "connLinkContact")]
    pub link: ContactFullLink,
}

#[derive(Debug, serde::Deserialize)]
pub struct ContactFullLink {
    #[serde(rename = "connFullLink")]
    pub full_link: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactRequest {
    pub contact_request_id: usize,
    pub agent_invitation_id: String,
    #[serde(rename = "userContactLinkId_")]
    pub user_contact_link_id: usize,
    #[serde(rename = "cReqChatVRange")]
    pub version_range: VersionRange,
    pub local_display_name: String,
    pub profile_id: usize,
    pub profile: Profile,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "xContactId")]
    pub contact_id: String,
    pub pq_support: bool,
}
