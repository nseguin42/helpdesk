use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::entity::Entity;

pub struct Comment {
    pub text: String,
    pub created: OffsetDateTime,
    pub author_id: Uuid,
    pub ticket_id: Uuid,
    pub id: Uuid,
}

impl Entity for Comment {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl Comment {
    pub fn new(body: String, author_id: Uuid, ticket_id: Uuid) -> Self {
        Self {
            text: body,
            created: OffsetDateTime::now_utc(),
            author_id,
            ticket_id,
            id: Uuid::new_v4(),
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.text = text;
    }
}
