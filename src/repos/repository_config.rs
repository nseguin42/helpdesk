use log::error;

use crate::utils::error::Error;

pub struct RepositoryConfig {
    pub(crate) connection_string: String,
}

impl RepositoryConfig {
    pub fn new(connection_string: String) -> Self {
        RepositoryConfig { connection_string }
    }

    pub async fn connect(&self) -> Result<tokio_postgres::Client, Error> {
        let (client, connection) =
            tokio_postgres::connect(&self.connection_string, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Connection error: {}", e);
            }
        });
        Ok(client)
    }
}
