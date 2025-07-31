#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteFolder {
    pub note_folder_id: usize,
    pub user_id: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "chatTs")]
    pub chat_timestamp: chrono::DateTime<chrono::Utc>,
    pub favorite: bool,
    pub unread: bool,
}
