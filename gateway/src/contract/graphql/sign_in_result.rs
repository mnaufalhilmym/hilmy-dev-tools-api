use async_graphql::Object;

pub struct SignInResult {
    pub token: String,
}

#[Object]
impl SignInResult {
    async fn token(&self) -> &str {
        &self.token
    }
}
