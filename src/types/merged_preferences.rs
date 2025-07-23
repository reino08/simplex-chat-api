use crate::types::Permission;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedPreferences {
    /// Self deleting messages.
    pub timed_messages: MergedPreference,
    /// Marking a message as deleted for 24 hours before being permanently deleted.
    pub full_delete: MergedPreference,
    /// Reacting with emojis to another message under it.
    pub reactions: MergedPreference,
    /// Sending voice messages in chat.
    pub voice: MergedPreference,
    /// Initiating telephone calls.
    pub calls: MergedPreference,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedPreference {
    pub enabled: MergedPreferenceEnabled,
    pub user_preference: MergedPreferenceUser,
    pub contact_preference: Permission,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedPreferenceEnabled {
    pub for_user: bool,
    pub for_contact: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MergedPreferenceUser {
    User { preference: Permission },
}
