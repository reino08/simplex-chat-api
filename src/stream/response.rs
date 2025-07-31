use crate::types::{
    Chat, Contact, ContactLink, FileDescriptor, FileTransfer, GroupInfo, GroupMemberCount, Message,
    Reaction, TerminalEvent, User, UserWithUnreadCount, errors::ChatError,
};

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    #[serde(rename = "corrId")]
    pub id: Option<String>,
    #[serde(rename = "resp")]
    pub data: ResponseData,
}

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("utf8 parse error: {0}")]
    UTF8Error(#[from] std::str::Utf8Error),
    #[error("deserialization error: {0}")]
    JSONError(#[from] serde_json::Error),
    #[error("unknown variant")]
    UnknownVariantError,
}

impl Response {
    pub(crate) fn parse(str: &str) -> Result<Self, ResponseError> {
        serde_json::from_str::<Self>(str)
            .inspect_err(|err| log::error!("Failed to decode response: {err}: {str}"))
            .map_err(ResponseError::JSONError)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ResponseData {
    /// ### `/u`, `/user`
    ActiveUser {
        user: User,
    },
    #[serde(rename = "chatCmdError")]
    ChatCommandError {
        chat_error: ChatError,
    },
    ChatError {
        chat_error: ChatError,
    },
    /// ### `/h`, `/help`
    /// Does not contain useful information.
    ChatHelp {
        help_section: String,
    },
    ChatItemReaction {
        user: User,
        added: bool,
        reaction: Box<Reaction>,
    },
    ChatItemsStatusesUpdated {
        user: User,
        chat_items: Vec<Message>,
    },
    ChatItemUpdated {
        user: User,
        chat_item: Message,
    },
    /// ### `/cs`, `/chats`
    /// All of the chats (contacts + groups) for the user.
    Chats {
        chats: Vec<Chat>,
    },
    /// ### `/_stop`
    /// The client was stopped and the process terminated.
    ChatStopped,
    /// ### `/contacts`
    ContactsList {
        user: User,
        contacts: Vec<Contact>,
    },
    ContactsDisconnected {
        server: String,
        contact_refs: Vec<serde_json::Value>,
    },
    ContactsSubscribed {
        server: String,
        contact_refs: Vec<serde_json::Value>,
    },
    ContactSubSummary {
        user: User,
        contact_subscriptions: Vec<serde_json::Value>,
    },
    GroupsList {
        user: User,
        groups: Vec<(GroupInfo, GroupMemberCount)>,
    },
    NewChatItems {
        user: User,
        chat_items: Vec<Message>,
    },
    #[serde(rename = "rcvFileDescrReady")]
    ReceiveFileDescriptorReady {
        user: User,
        chat_item: Message,
        #[serde(rename = "rcvFileTransfer")]
        file_transfer: FileTransfer,
        #[serde(rename = "rcvFileDescr")]
        file_descriptor: FileDescriptor,
    },
    TerminalEvent {
        data: TerminalEvent,
    },
    /// ### `/users`
    /// Gets local profiles. See `/contacts` instead.
    UsersList {
        users: Vec<UserWithUnreadCount>,
    },
    UserContactLink {
        user: User,
        contact_link: ContactLink,
    },
    UserContactSubSummary {
        user: User,
        user_contact_subscriptions: Vec<serde_json::Value>, // userContact: { userContactLinkId: number, connReqContact: String }
    },
}
