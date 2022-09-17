use crate::{api::provider_api::WeatherProvider, error::WeatherError};

/// Contains an API every kind of storage should implement.
/// The entity the storage is currently implemented to store is one which implements [`WeatherProvider`].
pub trait Storage {
    /// Get all providers.
    fn get_all(&self) -> Vec<&dyn WeatherProvider>;

    /// Add new provider.
    fn add(&mut self, provider: Box<dyn WeatherProvider>) -> Result<(), WeatherError>;

    /// Get a provider by its name.
    fn get(&mut self, key: &str) -> Option<&mut Box<dyn WeatherProvider>>;

    /// Delete provider by its name.
    fn delete(&mut self, key: &str) -> Result<(), WeatherError>;

    /// Set default entry.
    fn set_default_entry(&mut self, key: &str) -> Result<(), WeatherError>;

    /// Get default entry.
    fn get_default_entry(&mut self) -> Option<&mut Box<dyn WeatherProvider>>;
}
