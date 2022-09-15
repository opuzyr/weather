use std::{io};

use serde::{Serialize, Deserialize};
use thiserror::Error;


/// Represents internal errors.
#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum WeatherError {

    /// Parsing CLI commands failed.
    #[error("Failed to parse cli")]
    CliParserError(String),

    /// Failed to fetch report data.
    #[error("Failed to fetch report data")]
    ReportDataError,

    /// Something not ok with one implementing [`crate::storage::storage_api::Storage`].
    #[error("Failed to read config file")]
    ReadConfigFileError(String),

    /// No default provider set.
    #[error("No default provider")]
    NoDefaultProviderError,

    /// No API_KEY provided
    #[error("Failed to execute http request")]
    NoApiKeyError,
 

    /// HTTP call error
    #[error("Failed to execute http request")]
    HttpError(String),

    /// Location retrieval error. Provider specific.
    #[error("No location found to provide a report")]
    NoLocationFoundError,

    /// No report provided error. Provider specific.
    #[error("No report found for provided location")]
    NoReportFoundError

}

impl From<io::Error> for WeatherError {
    fn from(error: io::Error) -> Self {
        Self::ReadConfigFileError(error.to_string())
    }
}

impl From<serde_json::Error> for WeatherError {
    fn from(error: serde_json::Error) -> Self {
        Self::ReadConfigFileError(error.to_string())
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpError(error.to_string())
    }
}
