use std::io::Write;

use log::{debug, Level};
use pretty_env_logger::env_logger;
use pretty_env_logger::env_logger::Env;
use time::{format_description, OffsetDateTime};

use crate::utils::error::Error;

const LOG_FORMAT_TIMESTAMP: &str = "[hour]:[minute]:[second].[subsecond digits:3]";

pub async fn setup_logger() -> Result<(), Error> {
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));

    builder.format(|buf, record| {
        let level = match record.level() {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };

        let format = format_description::parse(LOG_FORMAT_TIMESTAMP).or_else(|error| {
            Err(Error::new(format!(
                "Logger error: Failed to parse format description: {}",
                error
            )))
        })?;

        let timestamp = OffsetDateTime::now_utc().format(&format).or_else(|error| {
            Err(Error::new(format!(
                "Logger error: Failed to format timestamp: {}",
                error
            )))
        })?;

        writeln!(buf, "{} [{}] {}", timestamp, level, record.args())
    });

    builder
        .try_init()
        .map_err(|error| Error::new(format!("Logger error: {}", error)))?;

    debug!("Logger initialized");

    Ok(())
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[actix_rt::test]
    async fn test_setup_logger() -> Result<(), Error> {
        setup_logger().await?;
        info!("Logger initialized");

        Ok(())
    }
}
