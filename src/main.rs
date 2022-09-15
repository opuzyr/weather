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
#[clap(
    version,
    about = "Provides temperature for specified city. "
)]
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
    List
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
        storage.add(Box::new(OpenWeatherProvider::new(
            "OpenWeather",
            None
        )))?;
        storage.add(Box::new(AccuweatherProvider::new(
            "Accuweather",
            None
        )))?;
    }
    Ok(())
}

fn main() -> Result<(), WeatherError> {
    let mut storage: Box<dyn Storage> = Box::new(JsonStorage::new(JSON_STORAGE_FILE)?);

    init_providers(&mut storage)?;

    match &Args::parse().command {
        Commands::Configure { provider_name } => {
            let provider = storage.get(provider_name);
            if let Some(mut provider) = provider.cloned() {
                let api_key =
                    rprompt::prompt_reply_stdout(&format!("{provider_name}'s API_KEY:")).unwrap();
                provider.set_api_key(&api_key);
                storage.add(provider)?;
                println!("API_KEY changed for {provider_name}");
            } else {
                return Err(WeatherError::CliParserError("No provider found".to_owned()));
            }
        }
        
        Commands::Get { address, date } => {
            let default_provider = storage.get_default_entry()?;
            let report = default_provider.get_report(address, *date)?;
            println!("{date}, {}: {report}", default_provider.get_name());
        }

        Commands::List => {
            let default_provider = storage.get_default_entry()?;
            storage.get_all().iter().for_each(|provider| {
                let is_default = provider.0.eq(&default_provider.get_name());
                println!(
                    "Name: {}, API_KEY: {}, default: {}",
                    provider.0,
                    provider
                        .1
                        .get_api_key()
                        .unwrap_or_else(|| "not set".to_owned()),
                        is_default
                );
            });
        }

        Commands::Default { provider_name } => {
            storage.set_default_entry(provider_name)?;
        }
    }

    Ok(())
}
