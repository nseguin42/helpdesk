use std::hash::{Hash, Hasher};

use uuid::Uuid;

pub trait Entity {
    fn get_id(&self) -> &Uuid;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Hash for dyn Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}
