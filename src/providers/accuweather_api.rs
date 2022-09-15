use chrono::NaiveDate;
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    api::provider_api::{WeatherProvider, WeatherReport},
    error::WeatherError,
};

/// [`WeatherProvider`] implementation for [`AccuweatherProvider`].
/// Naïve and no historical data support.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccuweatherProvider {
    /// Using sync implementation of reqwest since we are all sync. No need to store as well.
    #[serde(skip)]
    client: Client,
    provider_name: String,
    api_key: Option<String>,
}

impl AccuweatherProvider {
    pub fn new(provider_name: &str, api_key: Option<&str>) -> Self {
        AccuweatherProvider {
            client: Client::new(),
            provider_name: provider_name.to_owned(),
            api_key: api_key.map(str::to_string),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LocationKey {
    key: String,
    localized_name: String,
}

/// Internal representation of metric
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Metric {
    value: f32,
    unit: String,
}

/// Internal representaion of temperature.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Temperature {
    metric: Metric,
}

/// Internal json for temperature option.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TemperatureReport {
    temperature: Temperature,
}

#[typetag::serde]
impl WeatherProvider for AccuweatherProvider {
    fn get_name(&self) -> String {
        self.provider_name.clone()
    }

    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }

    fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_owned());
    }

    // Accuweather seems to have unreasonably low call rates for free accout to test the call...
    fn get_report(&self, address: &str, _date: NaiveDate) -> Result<WeatherReport, WeatherError> {
        let response = self.client.get(format!("http://dataservice.accuweather.com/locations/v1/cities/autocomplete?apikey={}&q={}", self.get_api_key().ok_or(WeatherError::NoApiKeyError)?, address)).send()?;
        if response.status() != StatusCode::OK {
            return Err(WeatherError::HttpError(response.text()?));
        }

        let locations: Vec<LocationKey> = response.json()?;

        if let Some(location) = locations.get(0) {
            let response = self
                .client
                .get(format!(
                    "http://dataservice.accuweather.com/currentconditions/v1/{}?apikey={}",
                    location.key,
                    self.get_api_key().unwrap()
                ))
                .send()?;
            let reports: Vec<TemperatureReport> = response.json()?;
            if let Some(report) = reports.get(0) {
                let report = format!(
                    "{} {}",
                    report.temperature.metric.value, report.temperature.metric.unit
                );
                Ok(WeatherReport { report })
            } else {
                Err(WeatherError::NoReportFoundError)
            }
        } else {
            Err(WeatherError::NoLocationFoundError)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use crate::{api::provider_api::WeatherProvider, error::WeatherError};

    use super::AccuweatherProvider;

    #[test]
    fn no_api_key_error_expected() {
        let provider = AccuweatherProvider::new("testprovider", None);
        let result = provider.get_report("foo", Local::now().date_naive());
        assert_eq!(Err(WeatherError::NoApiKeyError), result);
    }

    #[test]
    fn some_invalid_api_key_error_expected() {
        let provider = AccuweatherProvider::new("testprovider", Some("somekey"));
        let result = provider.get_report("foo", Local::now().date_naive());
        assert_eq!(Err(WeatherError::HttpError("{\"Code\":\"Unauthorized\",\"Message\":\"Api Authorization failed\",\"Reference\":\"/locations/v1/cities/autocomplete?apikey=somekey&q=foo\"}".to_owned())), result);
    }
}
