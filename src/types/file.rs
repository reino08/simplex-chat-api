#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTransfer {
    #[serde(rename = "fileId")]
    pub id: usize,
    #[serde(rename = "xftpRcvFile")]
    pub file: XftpReceiveFile,
    pub file_invitation: FileInvitation,
    pub file_status: FileStatus,
    pub sender_display_name: String,
    pub chunk_size: usize,
    pub cancelled: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct XftpReceiveFile {
    #[serde(rename = "rcvFileDescription")]
    pub descriptor: FileDescriptor,
    #[serde(rename = "agentRcvFileDeleted")]
    pub deleted: bool,
    #[serde(rename = "userApprovedRelays")]
    pub user_approved_relays: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInvitation {
    pub file_name: String,
    pub file_size: usize,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FileStatus {
    New,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileDescriptor {
    #[serde(rename = "fileDescrId")]
    pub id: usize,
    #[serde(rename = "fileDescrText")]
    pub text: String,
    #[serde(rename = "fileDescrPartNo")]
    pub part_number: usize,
    #[serde(rename = "fileDescrComplete")]
    pub complete: bool,
}
