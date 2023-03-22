use async_graphql::Object;
use uuid::Uuid;

pub struct Link {
    pub id: Uuid,
    pub title: String,
    pub short_url: String,
    pub long_url: String,
    pub visits: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[Object]
impl Link {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn short_url(&self) -> &str {
        &self.short_url
    }

    async fn long_url(&self) -> &str {
        &self.long_url
    }

    async fn visits(&self) -> &i32 {
        &self.visits
    }

    async fn created_at(&self) -> &str {
        &self.created_at
    }

    async fn updated_at(&self) -> &str {
        &self.updated_at
    }
}
