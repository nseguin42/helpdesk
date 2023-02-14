use std::process::id;

use time::OffsetDateTime;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::models::user::User;
use crate::repos::repository::Repository;
use crate::repos::repository_config::RepositoryConfig;
use crate::repos::statement_wrapper::StatementWrapper;
use crate::utils::error::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) enum UserRepositoryStatement {
    Get,
    GetMany,
    Add,
    Update,
    Delete,
}

impl UserRepositoryStatement {
    fn get(&self) -> &str {
        match self {
            UserRepositoryStatement::Get => "SELECT * FROM users WHERE id = $1",
            UserRepositoryStatement::GetMany => {
                "SELECT * FROM users ORDER BY created DESC LIMIT $1 OFFSET $2"
            }
            UserRepositoryStatement::Add => {
                "INSERT INTO users (id, name, created, updated) VALUES ($1, $2, $3, $4)"
            }
            UserRepositoryStatement::Update => {
                "UPDATE users SET name = $2, updated = $3 WHERE id = $1"
            }
            UserRepositoryStatement::Delete => "DELETE FROM users WHERE id = $1",
        }
    }
}

pub struct UserRepository {
    pub(crate) config: RepositoryConfig,

    statements: Vec<StatementWrapper>,
    client: Client,
}

impl UserRepository {
    pub async fn new(config: RepositoryConfig) -> Result<Self, Error> {
        let client = config.connect().await?;

        let mut statements = vec![
            StatementWrapper::new(UserRepositoryStatement::Get.get()),
            StatementWrapper::new(UserRepositoryStatement::GetMany.get()),
            StatementWrapper::new(UserRepositoryStatement::Add.get()),
            StatementWrapper::new(UserRepositoryStatement::Update.get()),
            StatementWrapper::new(UserRepositoryStatement::Delete.get()),
        ];

        for statement in statements.iter_mut() {
            statement.prepare(&client).await?;
            dbg!(statement);
        }

        let user_repository = UserRepository {
            config,
            statements,
            client,
        };

        Ok(user_repository)
    }
}

#[async_trait::async_trait]
impl Repository<User> for UserRepository {
    async fn get(&self, id: &Uuid) -> Result<User, Error> {
        let wrapper = &self.statements[UserRepositoryStatement::Get as usize];
        let statement = wrapper
            .get_prepared()
            .unwrap_or_else(|| panic!("Statement not prepared: {}", wrapper.get()));

        let row = self.client.query_one(statement, &[id]).await?;
        let user = User::from(row);

        Ok(user)
    }

    async fn get_many(&self, limit: i64, offset: i64) -> Vec<User> {
        let statement = self.statements[UserRepositoryStatement::GetMany as usize]
            .get_prepared()
            .unwrap();
        let rows = self
            .client
            .query(statement, &[&limit, &offset])
            .await
            .unwrap();

        let mut users = Vec::new();

        for row in rows {
            let user = User::from(row);
            users.push(user);
        }

        users
    }

    async fn add(&self, user: &User) -> Result<(), Error> {
        let statement = self.statements[UserRepositoryStatement::Add as usize]
            .get_prepared()
            .unwrap();
        let _ = self
            .client
            .execute(
                statement,
                &[&user.id, &user.name, &user.created, &user.updated],
            )
            .await?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), Error> {
        let statement = &self.statements[UserRepositoryStatement::Update as usize];
        dbg!(statement);
        let statement = statement.get_prepared().unwrap();
        let _ = self
            .client
            .execute(
                statement,
                &[
                    &user.id,
                    &user.name,
                    &Option::from(OffsetDateTime::now_utc()),
                ],
            )
            .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let statement = self.statements[UserRepositoryStatement::Delete as usize]
            .get_prepared()
            .unwrap();
        let _ = self.client.execute(statement, &[id]).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::static_env::get;

    #[actix_rt::test]
    async fn db_test_get_many() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let user_repository = UserRepository::new(config).await.unwrap();
        let users = user_repository.get_many(1, 0).await;
        assert!(users.len() > 0);
    }

    #[actix_rt::test]
    async fn db_test_create_user() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let user_repository = UserRepository::new(config).await.unwrap();

        let author_id = Uuid::parse_str("43ff49e9-0caf-42fa-8d0c-d7719dfc7229").unwrap();

        let user = User::new(Uuid::new_v4(), "Test user".to_string());
        let result = user_repository.add(&user).await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn db_test_update_user() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let user_repository = UserRepository::new(config).await.unwrap();
        let mut user = user_repository.get_many(1, 0).await[0].clone();
        user.name = user.name + " updated";
        let result = user_repository.update(&user).await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn db_test_delete_user() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let user_repository = UserRepository::new(config).await.unwrap();

        let users = user_repository.get_many(1, 0).await;
        let id = users[0].id;
        let result = user_repository.delete(&id).await;

        assert!(result.is_ok());
    }
}
