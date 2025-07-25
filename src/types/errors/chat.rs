#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ChatError {
    CommandError { message: String },
    Error { error_type: ChatErrorType },
}

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ChatErrorType {
    CommandError { message: String },
    ContactNotFound { contact_name: String },
}
