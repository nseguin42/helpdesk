use time::OffsetDateTime;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::models::ticket::Ticket;
use crate::repos::repository::Repository;
use crate::repos::repository_config::RepositoryConfig;
use crate::repos::statement_wrapper::StatementWrapper;
use crate::utils::error::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) enum TicketRepositoryStatement {
    Get,
    GetMany,
    Add,
    Update,
    Delete,
}

impl TicketRepositoryStatement {
    fn get(&self) -> &str {
        match self {
            TicketRepositoryStatement::Get => "SELECT * FROM tickets WHERE id = $1",
            TicketRepositoryStatement::GetMany => "SELECT * FROM tickets ORDER BY created DESC LIMIT $1 OFFSET $2",
            TicketRepositoryStatement::Add => "INSERT INTO tickets (id, title, description, created, updated, author_id) VALUES ($1, $2, $3, $4, $5, $6)",
            TicketRepositoryStatement::Update => "UPDATE tickets SET title = $2, description = $3, updated = $4 WHERE id = $1",
            TicketRepositoryStatement::Delete => "DELETE FROM tickets WHERE id = $1",
        }
    }
}

pub struct TicketRepository {
    pub(crate) config: RepositoryConfig,

    statements: Vec<StatementWrapper>,
    client: Client,
}

impl TicketRepository {
    pub async fn new(config: RepositoryConfig) -> Result<Self, Error> {
        let client = config.connect().await?;

        let mut statements = vec![
            StatementWrapper::new(TicketRepositoryStatement::Get.get()),
            StatementWrapper::new(TicketRepositoryStatement::GetMany.get()),
            StatementWrapper::new(TicketRepositoryStatement::Add.get()),
            StatementWrapper::new(TicketRepositoryStatement::Update.get()),
            StatementWrapper::new(TicketRepositoryStatement::Delete.get()),
        ];

        for statement in statements.iter_mut() {
            statement.prepare(&client).await?;
            dbg!(statement);
        }

        let ticket_repository = TicketRepository {
            config,
            statements,
            client,
        };

        Ok(ticket_repository)
    }
}

#[async_trait::async_trait]
impl Repository<Ticket> for TicketRepository {
    async fn get(&self, id: &Uuid) -> Result<Ticket, Error> {
        let wrapper = &self.statements[TicketRepositoryStatement::Get as usize];
        let statement = wrapper
            .get_prepared()
            .unwrap_or_else(|| panic!("Statement not prepared: {}", wrapper.get()));

        let row = self.client.query_one(statement, &[id]).await?;
        let ticket = Ticket::from(row);

        Ok(ticket)
    }

    async fn get_many(&self, limit: i64, offset: i64) -> Vec<Ticket> {
        let statement = self.statements[TicketRepositoryStatement::GetMany as usize]
            .get_prepared()
            .unwrap();
        let rows = self
            .client
            .query(statement, &[&limit, &offset])
            .await
            .unwrap();

        let mut tickets = Vec::new();

        for row in rows {
            let ticket = Ticket::from(row);
            tickets.push(ticket);
        }

        tickets
    }

    async fn add(&self, ticket: &Ticket) -> Result<(), Error> {
        let statement = self.statements[TicketRepositoryStatement::Add as usize]
            .get_prepared()
            .unwrap();
        let _ = self
            .client
            .execute(
                statement,
                &[
                    &ticket.id,
                    &ticket.title,
                    &ticket.description,
                    &ticket.created,
                    &ticket.updated,
                    &ticket.author_id,
                ],
            )
            .await?;

        Ok(())
    }

    async fn update(&self, ticket: &Ticket) -> Result<(), Error> {
        let statement = &self.statements[TicketRepositoryStatement::Update as usize];
        dbg!(statement);
        let statement = statement.get_prepared().unwrap();
        let _ = self
            .client
            .execute(
                statement,
                &[
                    &ticket.id,
                    &ticket.title,
                    &ticket.description,
                    &Option::from(OffsetDateTime::now_utc()),
                ],
            )
            .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let statement = self.statements[TicketRepositoryStatement::Delete as usize]
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
        let ticket_repository = TicketRepository::new(config).await.unwrap();
        let tickets = ticket_repository.get_many(1, 0).await;
        assert!(tickets.len() > 0);
    }

    #[actix_rt::test]
    async fn db_test_create_ticket() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let ticket_repository = TicketRepository::new(config).await.unwrap();

        let author_id = Uuid::parse_str("43ff49e9-0caf-42fa-8d0c-d7719dfc7229").unwrap();

        let ticket = Ticket::new(
            "Test ticket".to_string(),
            "Test ticket description".to_string(),
            author_id,
        );
        let result = ticket_repository.add(&ticket).await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn db_test_update_ticket() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let ticket_repository = TicketRepository::new(config).await.unwrap();
        let mut ticket = ticket_repository.get_many(1, 0).await[0].clone();
        ticket.title = ticket.title + " updated";

        let result = ticket_repository.update(&ticket).await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn db_test_delete_ticket() {
        let config = RepositoryConfig::new(get("db.url").unwrap());
        let ticket_repository = TicketRepository::new(config).await.unwrap();

        let id = Uuid::parse_str("43ff49e9-0caf-42fa-8d0c-d7719dfc7229").unwrap();
        let result = ticket_repository.delete(&id).await;

        assert!(result.is_ok());
    }
}
