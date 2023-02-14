use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::process::Output;

use tokio_postgres::{Client, Statement};

use crate::utils::error::Error;

#[derive(Clone)]
pub(crate) struct StatementWrapper {
    str: &'static str,
    prepared: Option<Statement>,
}

impl Debug for StatementWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatementWrapper")
            .field("str", &self.str)
            .field("prepared", &self.prepared.is_some())
            .finish()
    }
}

impl StatementWrapper {
    pub fn new(str: &'static str) -> Self {
        StatementWrapper {
            str,
            prepared: None,
        }
    }

    pub(crate) fn get(&self) -> &str {
        self.str
    }

    pub(crate) fn get_prepared(&self) -> Option<&Statement> {
        Option::from(self.prepared.as_ref().unwrap())
    }

    pub(crate) async fn prepare(&mut self, client: &Client) -> Result<(), Error> {
        let statement = client.prepare(&self.str).await?;
        self.prepared = Some(statement);
        Ok(())
    }
}
