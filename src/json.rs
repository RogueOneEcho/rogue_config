use crate::OptionsProvider;
use rogue_logging::Error;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const FILE_NAME: &str = "config.json";

pub struct JsonOptionsProvider;

impl OptionsProvider for JsonOptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, Error> {
        deserialize_from_file(&PathBuf::from(FILE_NAME))
    }
}

fn deserialize_from_file<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let file = File::open(path).map_err(|e| Error {
        action: "open options file".to_owned(),
        message: e.to_string(),
        domain: Some("file system".to_owned()),
        ..Error::default()
    })?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| Error {
        action: "deserialize options file".to_owned(),
        message: e.to_string(),
        domain: Some("deserialization".to_owned()),
        ..Error::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::example::Example;

    #[test]
    fn test() -> Result<(), Error> {
        // Arrange
        // Act
        let options = JsonOptionsProvider::get::<Example>()?;

        // Assert
        assert_eq!(options, Example::example());
        Ok(())
    }
}
