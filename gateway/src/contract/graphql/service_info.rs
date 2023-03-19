use async_graphql::Object;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct ServiceInfo {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ServiceInfo {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}
