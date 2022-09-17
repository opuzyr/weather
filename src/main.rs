mod api;
mod error;
mod providers;
mod storage;

use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use error::WeatherError;
use providers::{accuweather_api::AccuweatherProvider, openweather_api::OpenWeatherProvider};
use storage::storage_api::Storage;

use crate::storage::json_storage::JsonStorage;

static JSON_STORAGE_FILE: &str = "json_storage.json";

#[derive(Parser)]
#[clap(version, about = "Provides weather report for specified city. ")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// Commands supported by the app.
#[derive(Subcommand)]
enum Commands {
    /// Configure weather's provider.
    Configure {
        #[clap(value_name = "provider")]
        provider_name: String,
    },

    /// Show the weather for the provided address.
    Get {
        /// Address the weather is looked for
        #[clap(value_name = "address")]
        address: String,

        /// Optional date weather is looked for.
        #[clap(value_name = "date", parse(from_str=get_date), default_value = "now")]
        date: NaiveDate,
    },

    /// Set the default provider
    Default {
        /// The provider the app will set as default.
        #[clap(value_name = "provider")]
        provider_name: String,
    },

    /// List available providers.
    List,
}

fn get_date(date_string: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => Local::now().date_naive(),
    }
}

/// Init 2 default providers. Last added is considered as default one
fn init_providers(storage: &mut Box<dyn Storage>) -> Result<(), WeatherError> {
    if storage.get_all().is_empty() {
        storage.add(Box::new(AccuweatherProvider::new("Accuweather", None)))?;
        storage.add(Box::new(OpenWeatherProvider::new("OpenWeather", None)))?;
    }
    Ok(())
}

fn main() -> Result<(), WeatherError> {
    let mut storage: Box<dyn Storage> = Box::new(JsonStorage::new(JSON_STORAGE_FILE)?);
    init_providers(&mut storage)?;

    match &Args::parse().command {
        Commands::Configure { provider_name } => match storage.get(provider_name).cloned() {
            Some(mut provider) => {
                let api_key =
                    rprompt::prompt_reply_stdout(&format!("{provider_name}'s API_KEY:")).unwrap();
                provider.set_api_key(&api_key);
                storage.add(provider)?;
                println!("API_KEY changed for {provider_name}");
            }
            None => {
                eprintln!("Error: {}", WeatherError::NoSuchProviderError)
            }
        },

        Commands::Get { address, date } => match storage.get_default_entry() {
            Some(default_provider) => {
                let report = default_provider.get_report(address, *date)?;
                println!("{date}, {}: {report}", default_provider.get_name());
            }
            None => {
                eprintln!("Error: {}", WeatherError::NoDefaultProviderError)
            }
        },

        Commands::List => {
            let default_provider_name = storage
                .get_default_entry()
                .map(|f| f.get_name())
                .unwrap_or_default();
            storage.get_all().iter().for_each(|provider| {
                let is_default = provider.get_name().eq(&default_provider_name);
                println!(
                    "Provider: {}, API_KEY: {}, default: {is_default}",
                    provider.get_name(),
                    provider
                        .get_api_key()
                        .unwrap_or_else(|| "not set".to_owned())
                );
            });
        }

        Commands::Default { provider_name } => {
            storage.set_default_entry(provider_name)?;
        }
    }

    Ok(())
}
