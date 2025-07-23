#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub timed_messages: Permission,
    pub full_delete: Permission,
    pub reactions: Permission,
    pub voice: Permission,
    pub calls: Permission,
}

#[derive(Debug, serde::Deserialize)]
pub struct Permission {
    pub allow: PermissionOption,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionOption {
    /// Allowed only if the other side allow it
    Yes,
    /// Never allowed
    No,
    /// Always allowed for the other side, even if the other side don't allow it for them.
    Always,
}
