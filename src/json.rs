use crate::{ConfigError, OptionsProvider};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const FILE_NAME: &str = "config.json";

pub struct JsonOptionsProvider;

impl OptionsProvider for JsonOptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, ConfigError> {
        deserialize_from_file(&PathBuf::from(FILE_NAME))
    }
}

fn deserialize_from_file<T: DeserializeOwned>(path: &Path) -> Result<T, ConfigError> {
    let file = File::open(path).map_err(ConfigError::fs)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(ConfigError::json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::example::Example;

    #[test]
    fn test() -> Result<(), ConfigError> {
        // Arrange
        // Act
        let options = JsonOptionsProvider::get::<Example>()?;

        // Assert
        assert_eq!(options, Example::example());
        Ok(())
    }
}
