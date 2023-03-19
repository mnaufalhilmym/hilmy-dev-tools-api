use async_graphql::Object;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::model;

pub struct ServiceAddress {
    pub id: Uuid,
    pub service_id: Uuid,
    pub address: String,
    pub status: model::ServiceAddressStatus,
    pub last_used_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ServiceAddress {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn service_id(&self) -> &Uuid {
        &self.service_id
    }

    async fn address(&self) -> &str {
        &self.address
    }

    async fn status(&self) -> &model::ServiceAddressStatus {
        &self.status
    }

    async fn last_used_at(&self) -> &NaiveDateTime {
        &self.last_used_at
    }

    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}
