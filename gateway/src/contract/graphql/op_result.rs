use async_graphql::Object;

pub struct OpResult {
    pub is_success: bool,
}

#[Object]
impl OpResult {
    async fn is_success(&self) -> &bool {
        &self.is_success
    }
}
