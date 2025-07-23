#[derive(Clone, Debug)]
pub struct CreateMessage {
    pub content: String,
    pub recipient: Recipient,
}

#[derive(Clone, Debug)]
pub enum Recipient {
    Direct(String),
    Group(String),
}
