use std::fmt::{self, Debug};

use chrono::NaiveDate;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::error::WeatherError;


/// Struct every implementation of [`WeatherProvider`] should return as a response querying for report.
/// Titled in the name of Weather Report band.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeatherReport {
    pub report: String,
}

impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.report)
    }
}


/// The common trait for one who wants to implement it's own provider. 
#[typetag::serde]
pub trait WeatherProvider: Debug + DynClone {
    /// Get provider's name.
    fn get_name(&self) -> String;

    /// Get provider's API_KEY.
    fn get_api_key(&self) -> Option<String>;

    /// Set provider's API_KEY.
    fn set_api_key(&mut self, api_key: &str);

    /// Gets a report. HISTORICAL LOOKUP is not yet implemented.
    fn get_report(&self, address: &str, date: NaiveDate) -> Result<WeatherReport, WeatherError>;
}

dyn_clone::clone_trait_object!(WeatherProvider);
