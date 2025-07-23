use crate::types::{ShortGroupInfo, User};

#[derive(Debug, serde::Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum TerminalEvent {
    MemberSubSummary {
        user: User,
        member_subscriptions: Vec<serde_json::Value>,
    },
    PendingSubSummary {
        user: User,
        pending_subscriptions: Vec<serde_json::Value>,
    },
    GroupSubscribed {
        user: User,
        short_group_info: ShortGroupInfo,
    },
}
