use uuid::Uuid;

use crate::models::entity::Entity;

pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl Entity for User {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}
