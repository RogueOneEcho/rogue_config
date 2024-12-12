use rogue_logging::Error;
use serde::de::DeserializeOwned;

pub trait OptionsProvider {
    fn get<T: DeserializeOwned>() -> Result<T, Error>;
}