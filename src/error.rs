use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use ConfigError::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(clippy::absolute_paths)]
pub enum ConfigError {
    /// An error occured deserializing the config
    /// Includes the [`envy::Error`], [`serde_json::Error`] or [`serde_yaml::Error`] as a string
    Deserialization(String),
    /// An error occured opening the config file
    /// Includes the `std::io::Error` as a string
    FileSystem(String),
}

#[allow(clippy::absolute_paths)]
impl ConfigError {
    pub(crate) fn envy(error: envy::Error) -> Self {
        Deserialization(error.to_string())
    }

    pub(crate) fn fs(error: std::io::Error) -> Self {
        FileSystem(error.to_string())
    }

    pub(crate) fn json(error: serde_json::Error) -> Self {
        Deserialization(error.to_string())
    }

    pub(crate) fn yaml(error: serde_yaml::Error) -> Self {
        Deserialization(error.to_string())
    }
}

impl Display for ConfigError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Deserialization(message) => {
                format!("A deserialization error occured: {message}")
            }
            FileSystem(message) => {
                format!("A file system error occured: {message}")
            }
        };
        message.fmt(formatter)
    }
}
