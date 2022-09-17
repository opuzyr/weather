use std::{fmt::Debug, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents internal errors.
#[derive(Error, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum WeatherError {
    /// Parsing CLI commands failed.
    #[error("Failed to parse cli.")]
    CliParserError,

    /// Failed to fetch report data.
    #[error("Failed to fetch report data.")]
    ReportDataError,

    /// Something not ok with one implementing [`crate::storage::storage_api::Storage`].
    #[error("Failed to read config file.")]
    ReadConfigFileError,

    /// No default provider set.
    #[error("No provider is found in configuration.")]
    NoSuchProviderError,

    /// No default provider set.
    #[error("No default provider.")]
    NoDefaultProviderError,

    /// No API_KEY provided
    #[error("No API_KEY provided")]
    NoApiKeyError,

    /// HTTP call error
    #[error("Failed to execute http request")]
    HttpError(String),

    /// Location retrieval error. Provider specific.
    #[error("No location found to provide a report.")]
    NoLocationFoundError,

    /// No report provided error. Provider specific.
    #[error("No report found for provided location.")]
    NoReportFoundError,
}

// Since main returns a Result and Err is forced to impl Debug need to override Debug to show human redable errors
impl Debug for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CliParserError
            | Self::ReportDataError
            | Self::ReadConfigFileError
            | Self::NoSuchProviderError
            | Self::NoDefaultProviderError
            | Self::NoApiKeyError
            | Self::NoLocationFoundError
            | Self::NoReportFoundError => {
                write!(f, "{}", self)
            }
            Self::HttpError(message) => write!(f, "{}. {}", self, message),
        }
    }
}

impl From<clap::Error> for WeatherError {
    fn from(_: clap::Error) -> Self {
        Self::CliParserError
    }
}

impl From<io::Error> for WeatherError {
    fn from(_: io::Error) -> Self {
        Self::ReadConfigFileError
    }
}

impl From<serde_json::Error> for WeatherError {
    fn from(_: serde_json::Error) -> Self {
        Self::ReadConfigFileError
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpError(error.to_string())
    }
}
