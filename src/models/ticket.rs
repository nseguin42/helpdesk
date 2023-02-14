use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::models::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author_id: Uuid,
    pub created: OffsetDateTime,
    pub updated: Option<OffsetDateTime>,
}

impl Entity for Ticket {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl Ticket {
    pub fn new(title: String, description: String, author_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            author_id,
            created: OffsetDateTime::now_utc(),
            updated: None,
        }
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }
}

impl From<Row> for Ticket {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            created: row.get(1),
            updated: row.get(2),
            title: row.get(3),
            description: row.get(4),
            author_id: row.get(5),
        }
    }
}
