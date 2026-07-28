use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TMailbox {
    pub(crate) id: String,
    pub(crate) local_part: String,
    pub(crate) domain: String,
    pub(crate) comment: Option<String>,
    pub(crate) routes_to: String,
    pub(crate) created: String,
    pub(crate) deleted: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxesResponse {
    pub(crate) mailboxes: Vec<TMailbox>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxResponse {
    pub(crate) mailbox: TMailbox,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMailboxDetails {
    pub(crate) local_part: String,
    pub(crate) domain: String,
    pub(crate) comment: Option<String>,
    pub(crate) routes_to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMailboxRequest {
    pub(crate) mailbox: AddMailboxDetails,
}
