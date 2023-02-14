use config::Config;
use serde::Deserialize;

use crate::utils::error::Error;

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::Environment::with_prefix("HELPDESK").separator("_"))
        .add_source(config::File::with_name("config/config"))
        .build()
        .unwrap();
}

/// Get a configuration value from the static configuration object
pub fn get<'a, T: Deserialize<'a>>(key: &str) -> Result<T, Error> {
    CONFIG
        .get::<T>(key)
        .map_err(|error| Error::Config(error.to_string()))
}

/// Get all configuration values from the static configuration object
pub fn get_all() -> Result<Config, Error> {
    Ok(CONFIG.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get() -> Result<(), Error> {
        let value = get::<String>("SERVER_HOST")?;
        dbg!(value);

        Ok(())
    }

    #[test]
    fn test_get_all() -> Result<(), Error> {
        let config = get_all()?;
        dbg!(config);

        Ok(())
    }
}
