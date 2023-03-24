use async_graphql::Object;
use uuid::Uuid;

pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[Object]
impl Account {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn created_at(&self) -> &str {
        &self.created_at
    }

    async fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

pub struct SignInResult {
    pub token: String,
}

#[Object]
impl SignInResult {
    async fn token(&self) -> &str {
        &self.token
    }
}
