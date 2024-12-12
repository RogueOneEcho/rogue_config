use crate::OptionsProvider;
use rogue_logging::Error;
use serde::de::DeserializeOwned;

pub struct EnvironmentOptionsProvider;

impl OptionsProvider for EnvironmentOptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, Error> {
        envy::from_env::<T>().map_err(|e| Error {
            action: "deserialize environment variables".to_owned(),
            message: e.to_string(),
            domain: Some("deserialization".to_owned()),
            ..Error::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::example::Example;
    use std::env;

    #[test]
    fn test() -> Result<(), Error> {
        // Arrange
        env::set_var("STRING", "Hello, world!");
        env::set_var("FILE_PATH", "/path/to/file");
        env::set_var("ISIZE", "-12345");
        env::set_var("USIZE", "12345");
        env::set_var("F32", "-12345.6789");
        env::set_var("BOOL", "true");
        env::set_var("VEC", "One,Two,Three");
        env::set_var("ENUMERABLE", "One");
        env::set_var("UNMATCHED", "value");

        // Act
        let options = EnvironmentOptionsProvider::get::<Example>()?;

        // Assert
        assert_eq!(options, Example::flat());
        Ok(())
    }
}
