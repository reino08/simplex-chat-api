/// Represents a local profile. Other users are represented as contacts.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: usize,
    pub agent_user_id: String,
    pub user_contact_id: usize,
    /// Unique display name used for commands. A numerical suffix is added to prevent duplicates (e.g. `User_2`).
    pub local_display_name: String,
    pub profile: super::Profile,
    pub full_preferences: super::Preferences,
    pub active_user: bool,
    pub active_order: usize,
    #[serde(rename = "showNtfs")]
    pub show_notifications: bool,
    /// Notifies when message are read to individual contacts
    #[serde(rename = "sendRcptsContacts")]
    pub send_receipts_contacts: bool,
    /// Notifies when message are read to small groups (less than 20 members)
    #[serde(rename = "sendRcptsSmallGroups")]
    pub send_receipts_small_groups: bool,
    /// Timestamp of the last time the profile has been updated
    pub user_member_profile_updated_at: chrono::DateTime<chrono::Utc>,
}

/// A wrapper around the [`User`] type but with the amount of unread notifications.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithUnreadCount {
    pub user: User,
    pub unread_count: usize,
}
