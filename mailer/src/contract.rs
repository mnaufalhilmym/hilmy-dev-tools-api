use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MailReq {
    pub to: String,
    pub subject: String,
    pub body: String,
}
