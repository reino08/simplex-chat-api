#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// A local incrementing identifier.
    #[serde(rename = "profileId")]
    pub id: usize,
    /// The first word in a username.
    ///
    /// The CLI client shows this before the full name.
    ///
    /// The desktop client shows in this before a slash before the full name in the contact list,
    /// and above the full name in the selected conversation screen.
    pub display_name: String,
    /// The rest of the words in a username after the first.
    ///
    /// The CLI client shows this in parenthesis after the display name.
    ///
    /// The Desktop client shows this after a slash after the display name,
    /// and below the display name in the selected conversation screen.
    pub full_name: String,
    /// A data blob URL representing the profile picture.
    #[serde(default)]
    pub image: Option<String>,
    /// A nickname, not shared with others.
    pub local_alias: String,
    /// Reusable contact invite link.
    ///
    /// Contacts may choose to not share this field, even if they have created one.
    #[serde(default)]
    pub contact_link: Option<String>,
    /// Contact preferences.
    ///
    /// This field will be [`None`] for local profiles, see the [`crate::types::User::full_preferences`] field instead.
    #[serde(default)]
    pub preferences: Option<super::Preferences>,
}
