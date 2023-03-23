use async_graphql::Object;
use uuid::Uuid;

pub struct Apprepo {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub link: String,
    pub created_at: String,
    pub updated_at: String,
}

#[Object]
impl Apprepo {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn icon(&self) -> &str {
        &self.icon
    }

    async fn link(&self) -> &str {
        &self.link
    }

    async fn created_at(&self) -> &str {
        &self.created_at
    }

    async fn updated_at(&self) -> &str {
        &self.updated_at
    }
}
