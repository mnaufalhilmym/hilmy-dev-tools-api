use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountChangeEmail {
    pub old_email: String,
    pub new_email: String,
    pub verify_code: String,
}
