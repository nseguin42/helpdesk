use std::fmt::{Display, Formatter};

use config::ConfigError;
use time::error::InvalidFormatDescription;

#[derive(Debug)]
pub enum Error {
    Model(String),
    Repo(String),
    Server(String),
    Config(String),
    Unspecified(String),
}

impl Error {
    pub fn new(message: String) -> Error {
        Error::Unspecified(message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Model(message) => write!(f, "Model error: {}", message),
            Error::Repo(message) => write!(f, "Repo error: {}", message),
            Error::Config(message) => write!(f, "Config error: {}", message),
            Error::Server(message) => write!(f, "Server error: {}", message),
            Error::Unspecified(message) => write!(f, "Unspecified error: {}", message),
        }
    }
}

impl From<Error> for String {
    fn from(error: Error) -> String {
        match error {
            Error::Model(message) => format!("Model error: {}", message),
            Error::Repo(message) => format!("Repo error: {}", message),
            Error::Config(message) => format!("Config error: {}", message),
            Error::Server(message) => format!("Server error: {}", message),
            Error::Unspecified(message) => format!("Unspecified error: {}", message),
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl From<actix_web::Error> for Error {
    fn from(error: actix_web::Error) -> Error {
        Error::Server(error.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Error {
        Error::Model(error.to_string())
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Error {
        Error::Config(error.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Error {
        Error::Unspecified(error.to_string())
    }
}

impl From<InvalidFormatDescription> for Error {
    fn from(error: InvalidFormatDescription) -> Error {
        Error::Unspecified(error.to_string())
    }
}

impl From<time::error::Format> for Error {
    fn from(error: time::error::Format) -> Error {
        Error::Unspecified(error.to_string())
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Error {
        Error::Repo(error.to_string())
    }
}
