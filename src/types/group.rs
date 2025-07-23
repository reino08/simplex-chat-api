use crate::types::{Connection, Profile, VersionRange};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub group_id: usize,
    pub local_display_name: String,
    /// Use [`Self::fullGroupPreferences`] instead.
    pub group_profile: serde_json::Value,
    pub local_alias: String,
    pub full_group_preferences: Box<GroupPreferences>,
    pub membership: Box<GroupMember>,
    pub chat_settings: serde_json::Value, // "enableNtfs": "all", "favorite": false
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "chatTs")]
    pub chat_timestamp: chrono::DateTime<chrono::Utc>,
    pub user_member_profile_sent_at: chrono::DateTime<chrono::Utc>,
    pub chat_tags: Vec<serde_json::Value>,
    pub members_require_attention: usize,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPreferences {
    pub timed_messages: serde_json::Value,
    pub direct_messages: serde_json::Value,
    pub full_delete: serde_json::Value,
    pub reactions: serde_json::Value,
    pub voice: serde_json::Value,
    pub files: serde_json::Value,
    pub simplex_links: serde_json::Value,
    pub reports: serde_json::Value,
    pub history: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub group_member_id: usize,
    pub group_id: usize,
    pub member_id: String,
    pub member_role: String,
    pub member_category: String,
    pub member_status: String,
    pub member_settings: serde_json::Value,
    pub blocked_by_admin: bool,
    pub invited_by: serde_json::Value,
    pub invited_by_group_member_id: Option<usize>,
    pub local_display_name: String,
    pub member_profile: Profile,
    pub member_contact_id: Option<usize>,
    pub member_contact_profile_id: usize,
    #[serde(rename = "activeConn")]
    pub active_connection: Option<Connection>,
    #[serde(rename = "memberChatVRange")]
    pub member_version_range: VersionRange,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShortGroupInfo {
    #[serde(rename = "groupId")]
    pub id: usize,
    #[serde(rename = "groupName")]
    pub name: String,
    #[serde(rename = "membershipStatus")]
    pub status: GroupMemberStatus,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupMemberStatus {
    Connected,
}
