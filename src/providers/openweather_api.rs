use std::fmt::Debug;

use chrono::NaiveDate;
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    api::provider_api::{WeatherProvider, WeatherReport},
    error::WeatherError,
};

/// [`WeatherProvider`] implementation for [`OpenWeatherProvider`].
/// Naïve and no historical data support.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenWeatherProvider {
    /// Using sync implementation of reqwest since we are all sync. No need to store as well.
    #[serde(skip)]
    client: Client,
    provider_name: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    name: String,
    lat: f32,
    lon: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Report {
    main: Main,
}

impl OpenWeatherProvider {
    pub fn new(provider_name: &str, api_key: Option<&str>) -> Self {
        OpenWeatherProvider {
            client: Client::new(),
            provider_name: provider_name.to_owned(),
            api_key: api_key.map(str::to_string),
        }
    }
}

#[typetag::serde]
impl WeatherProvider for OpenWeatherProvider {
    fn get_name(&self) -> String {
        self.provider_name.clone()
    }

    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }

    fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_owned());
    }

    fn get_report(&self, address: &str, _date: NaiveDate) -> Result<WeatherReport, WeatherError> {
        let response = self
            .client
            .get(format!(
                "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
                address,
                self.get_api_key().ok_or(WeatherError::NoApiKeyError)?
            ))
            .send()?;
        if response.status() != StatusCode::OK {
            return Err(WeatherError::HttpError(response.text()?));
        }
        let locations: Vec<Location> = response.json()?;

        if let Some(location) = locations.get(0) {
            let response = self
                .client
                .get(format!(
                    "https://api.openweathermap.org/data/2.5/weather?units=metric&lat={}&lon={}&appid={}",
                    location.lat,
                    location.lon,
                    self.get_api_key().ok_or(WeatherError::NoApiKeyError)?
                ))
                .send()?;

            let report: Report = response.json()?;
            Ok(WeatherReport {
                report: format!("{} C", report.main.temp),
            })
        } else {
            Err(WeatherError::NoLocationFoundError)
        }
    }
}
