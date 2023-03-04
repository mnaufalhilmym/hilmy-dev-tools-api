use async_graphql::Object;

pub struct OpRes {
    pub is_success: bool,
}

#[Object]
impl OpRes {
    async fn is_success(&self) -> &bool {
        &self.is_success
    }
}
