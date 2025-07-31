#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ChatError {
    CommandError {
        message: String,
    },
    Error {
        error_type: ChatErrorType,
    },
    ErrorAgent {
        agent_error: serde_json::Value,
        #[serde(rename = "connectionEntity_")]
        connection_entity: serde_json::Value,
    },
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
