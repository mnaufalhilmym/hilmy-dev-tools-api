use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountSignUp {
    pub email: String,
    pub password: String,
    pub verify_code: String,
}
