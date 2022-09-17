use std::{collections::HashMap, fs::OpenOptions, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

use crate::{api::provider_api::WeatherProvider, error::WeatherError};

use super::storage_api::Storage;

/// JSON storage implementation to hold provider entries in json file.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JsonStorage {
    path: String,
    configs: HashMap<String, Box<dyn WeatherProvider>>,
    default: Option<String>,
}

impl JsonStorage {
    pub fn new(path: &str) -> Result<Self, WeatherError> {
        let file_exists = Path::new(path).exists();
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create_new(!file_exists)
            .open(path)?;

        if file_exists && file.metadata()?.len() > 0 {
            Ok(serde_json::from_reader(BufReader::new(&file))?)
        } else {
            Ok(JsonStorage {
                path: path.to_owned(),
                configs: HashMap::new(),
                default: None,
            })
        }
    }

    fn save(&self) -> Result<(), WeatherError> {
        let file_exists = Path::new(&self.path).exists();
        let file = OpenOptions::new()
            .write(true)
            .create_new(!file_exists)
            .truncate(true)
            .open(&self.path)?;
        serde_json::to_writer(&file, &self)?;
        Ok(())
    }
}

impl Storage for JsonStorage {
    fn get_all(&self) -> Vec<&dyn WeatherProvider> {
        self.configs
            .values()
            .into_iter()
            .map(|p| p.as_ref())
            .collect()
    }

    fn add(
        &mut self,
        provider: Box<dyn WeatherProvider>,
    ) -> Result<(), crate::error::WeatherError> {
        self.configs.insert(provider.get_name(), provider);

        self.save()?;
        Ok(())
    }

    fn get(&mut self, key: &str) -> Option<&mut Box<dyn WeatherProvider>> {
        self.configs.get_mut(key)
    }

    fn delete(&mut self, key: &str) -> Result<(), WeatherError> {
        self.configs.remove(key);
        if self.default == Some(key.to_owned()) {
            self.default = None
        }
        self.save()?;
        Ok(())
    }

    fn set_default_entry(&mut self, key: &str) -> Result<(), WeatherError> {
        self.default = Some(
            self.get(key)
                .map(|provider| provider.get_name())
                .ok_or(WeatherError::NoSuchProviderError)?,
        );
        self.save()?;
        Ok(())
    }

    fn get_default_entry(&mut self) -> Option<&mut Box<dyn WeatherProvider>> {
        self.default.clone().map_or_else(|| None, |f| self.get(&f))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::{providers::openweather_api::OpenWeatherProvider, storage::storage_api::Storage};

    use super::JsonStorage;

    type TestResult<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    // Need to sync test threads to avoid "File already exists"
    static M: Mutex<()> = Mutex::new(());

    static STORAGE_JSON_FILE: &str = "test_json_storage.json";

    #[test]
    fn empty_storage_ok() -> TestResult {
        let _lock = M.lock()?;

        let storage: Box<dyn Storage> = Box::new(JsonStorage::new(STORAGE_JSON_FILE).unwrap());
        assert_eq!(0, storage.get_all().len());
        Ok(())
    }

    #[test]
    fn only_one_config_ok() -> TestResult {
        let _lock = M.lock()?;

        let provider_name = "Weather Provider A";
        let mut storage: Box<dyn Storage> = Box::new(JsonStorage::new(STORAGE_JSON_FILE).unwrap());
        let provider = Box::new(OpenWeatherProvider::new(provider_name, None));

        storage.add(provider).unwrap();
        assert_eq!(1, storage.get_all().len());

        let config = storage.get("Weather Provider A").unwrap();
        assert_eq!(None, config.get_api_key());

        storage.delete(provider_name).unwrap();

        let not_existing = storage.get("Provider Does Not Exist");
        assert!(not_existing.is_none());
        Ok(())
    }

    #[test]
    fn delete_ok() -> TestResult {
        let _lock = M.lock()?;

        let provider_name = "Weather Provider A for deletion";
        let name_cloned = provider_name.clone();
        let mut storage: Box<dyn Storage> = Box::new(JsonStorage::new(STORAGE_JSON_FILE).unwrap());

        let provider = OpenWeatherProvider::new(provider_name, None);
        storage.add(Box::new(provider)).unwrap();

        let config = storage.get(&name_cloned).unwrap();
        assert_eq!(None, config.get_api_key());

        storage.delete(&name_cloned).unwrap();

        let result = storage.get_default_entry();
        assert!(result.is_none());

        let config = storage.get(&name_cloned);
        assert!(config.is_none());
        Ok(())
    }
}
