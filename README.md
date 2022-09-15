# Weather
Yet another CLI weather app.

## Description

This is a simple CLI application written in Rust to grab weather data from <b>OpenWeather</b> and <b>Accuweather</b> 
providers. Currently the only one entry report contains of is temperature. Also no historical records provided so there is no possibility to get a report for specified date.


## Test
---
To run tests execute: 
```
cargo test
```

## Build
---
To build the application run:
```
cargo build --release
```

## Usage
---

### List configuration:

`weather list` - lists the current configuration. App shows you provider's name, **API_KEY** and whether it is default.

```
Name: OpenWeather, API_KEY: not set, default: true
Name: Accuweather, API_KEY: not set, default: false

```
### Configure API_KEY:
---

To use the application it needs to be configured to have API_KEY for at least one provider which is considered as default: 

`weather configure <provider>`

For instance, if you want to set **API_KEY** for OpenWeather run:

`weather configure OpenWeather`

Application will prompt you to enter **API_KEY** for OpenWeather:
```
Accuweather's API_KEY:
````
>***Note**: The last provider **API_KEY** is configured for automatically becomes default one.*

Configuration is saved into **json_storage.json** file. It is possible to manually edit the content to set **API_KEY** and make a provider default one:

```json
{
  "path": "json_storage.json",
  "configs": {
    "Accuweather": {
      "AccuweatherProvider": {
        "provider_name": "Accuweather",
        "api_key": null
      }
    },
    "OpenWeather": {
      "OpenWeatherProvider": {
        "provider_name": "OpenWeather",
        "api_key": null
      }
    }
  },
  "default": "OpenWeather"
}

```



### Setting default provider:
---
To set the default provider the weather report will be grabbed from run:

`weather default <provider>`

For instance, to set Accuweather as a default run:

`weather default Accuweather`

### Getting report
---

Once the app is configured (**API_KEY** is set) it is time to grab some data from default provider:

`weather get <address>`

For instance, if you need to get a report for Lviv, UA, run:

`weather get Lviv`

```
2022-09-15, OpenWeather: 15.88 C
```
