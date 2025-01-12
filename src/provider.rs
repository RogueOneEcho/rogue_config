use crate::ConfigError;
use serde::de::DeserializeOwned;

pub trait OptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, ConfigError>;
}
