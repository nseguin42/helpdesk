use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::models::entity::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created: OffsetDateTime,
    pub updated: Option<OffsetDateTime>,
}

impl Entity for User {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        User {
            id: row.get("id"),
            name: row.get("name"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl User {
    pub fn new(id: Uuid, name: String) -> Self {
        User {
            id,
            name,
            created: OffsetDateTime::now_utc(),
            updated: None,
        }
    }
}
