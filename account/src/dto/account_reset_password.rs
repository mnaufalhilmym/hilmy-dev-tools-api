use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountResetPassword {
    pub verify_code: String,
}
