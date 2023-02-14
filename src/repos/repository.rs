use uuid::Uuid;

use crate::models::entity::Entity;
use crate::utils::error::Error;

#[async_trait::async_trait]
pub(crate) trait Repository<T: Entity> {
    async fn get(&self, id: &Uuid) -> Result<T, Error>;
    async fn get_many(&self, limit: i64, offset: i64) -> Vec<T>;
    async fn add(&self, entity: &T) -> Result<(), Error>;
    async fn update(&self, entity: &T) -> Result<(), Error>;
    async fn delete(&self, id: &Uuid) -> Result<(), Error>;
}
