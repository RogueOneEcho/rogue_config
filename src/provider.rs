use serde::de::DeserializeOwned;
use crate::ConfigError;

pub trait OptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, ConfigError>;
}
