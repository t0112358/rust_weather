use std::{
	error::Error,
	fmt::{self, Display},
};

use reqwest::{blocking as reqwest_blocking, IntoUrl};

use serde_json::Value;

use crate::Config;

#[derive(Debug)]
pub struct Location {
	lat: f64,
	lon: f64,
}

#[derive(Debug, Clone)]
pub enum WeatherUnit {
	Imperial,
	Metric,
}

pub type Result<T> = std::result::Result<T, WeatherResultError>;

impl WeatherUnit {
	fn to_param(&self) -> String {
		self.to_string().to_lowercase()
	}
	fn to_temp_unit(&self) -> &str {
		match self {
			WeatherUnit::Imperial => "°F",
			WeatherUnit::Metric => "°C",
		}
	}
	fn to_pressure_unit(&self) -> &str {
		match self {
			WeatherUnit::Imperial => "PSI",
			WeatherUnit::Metric => "Pa",
		}
	}
}

impl Display for WeatherUnit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			WeatherUnit::Imperial => write!(f, "Imperial"),
			WeatherUnit::Metric => write!(f, "Metric"),
		}
	}
}

impl From<String> for WeatherUnit {
	fn from(s: String) -> WeatherUnit {
		match s.as_str() {
			"imperial" => WeatherUnit::Imperial,
			"metric" => WeatherUnit::Metric,
			other => panic!("Invalid type {}", other),
		}
	}
}

#[derive(Debug)]
pub struct WeatherResult {
	temp: f64,
	temp_feels_like: f64,
	temp_min: f64,
	temp_max: f64,
	pressure: f64,
	humidity: f64,
	unit: WeatherUnit,
}

impl WeatherResult {
	fn generate_weather_report(&self) -> String {
		format!(
			"
Weather report for today:
  Temperature: {} {temp_unit}  Feels like: {} {temp_unit}
  Min: {} {temp_unit}  Max: {} {temp_unit}
  Pressure: {} {pressure_unit}  Humidity: {}%
  All units are in the {} system",
			self.temp,
			self.temp_feels_like,
			self.temp_min,
			self.temp_max,
			self.pressure,
			self.humidity,
			self.unit.to_string().to_lowercase(),
			temp_unit = self.unit.to_temp_unit(),
			pressure_unit = self.unit.to_pressure_unit(),
		)
	}
}

impl fmt::Display for WeatherResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{}", self.generate_weather_report())
	}
}

#[derive(Debug)]
pub enum WeatherResultError {
	///Something went wrong getting the request
	RequestError(reqwest::Error),
	///No result was found
	NoResult,
	///Required fields were missing
	MissingFields,
}

impl Error for WeatherResultError {}

impl fmt::Display for WeatherResultError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::RequestError(err) => {
				write!(f, "Something went wrong getting the request: {}", err)
			}
			Self::NoResult => write!(f, "No result was found"),
			Self::MissingFields => write!(f, "Required fields were missing"),
		}
	}
}

impl From<reqwest::Error> for WeatherResultError {
	fn from(error: reqwest::Error) -> Self {
		WeatherResultError::RequestError(error)
	}
}

fn api_request<U: IntoUrl>(url: U) -> Result<Value> {
	let client = reqwest_blocking::Client::new();

	let result: Value = client.get(url).send()?.json()?;

	Ok(result)
}

fn parse_location(value: Value) -> Result<Location> {
	let results = match value {
		Value::Array(array) => array,
		_ => return Err(WeatherResultError::NoResult),
	};

	let result = match results.get(0) {
		Some(Value::Object(object)) => object,
		_ => return Err(WeatherResultError::NoResult),
	};

	let lat = result.get("lat");
	let long = result.get("lon");

	if lat.is_none() || long.is_none() {
		return Err(WeatherResultError::MissingFields);
	}

	Ok(Location {
		lat: lat.unwrap().as_f64().unwrap(),
		lon: long.unwrap().as_f64().unwrap(),
	})
}

fn get_location(config: &Config) -> Result<Location> {
	let url = format!(
		"https://api.openweathermap.org/geo/1.0/direct?q={location}&limit={limit}&appid={key}",
		location = config.location,
		limit = 1,
		key = config.key,
	);

	let results = api_request(url)?;

	let location = parse_location(results)?;

	Ok(location)
}

fn parse_weather(config: &Config, value: Value) -> Result<WeatherResult> {
	let result = match value {
		Value::Object(object) => object,
		_ => return Err(WeatherResultError::NoResult),
	};

	let main = match result.get("main") {
		Some(Value::Object(object)) => object,
		_ => {
			return Err(WeatherResultError::MissingFields);
		}
	};

	Ok(WeatherResult {
		temp: main.get("temp").unwrap().as_f64().unwrap(),
		temp_feels_like: main.get("feels_like").unwrap().as_f64().unwrap(),
		temp_min: main.get("temp_min").unwrap().as_f64().unwrap(),
		temp_max: main.get("temp_max").unwrap().as_f64().unwrap(),
		pressure: main.get("pressure").unwrap().as_f64().unwrap(),
		humidity: main.get("humidity").unwrap().as_f64().unwrap(),
		unit: config.unit.clone(),
	})
}

fn get_weather_at_location(config: &Config, location: Location) -> Result<WeatherResult> {
	//Handle result individually
	let url = format!(
		"https://api.openweathermap.org/data/2.5/weather?\
		lat={lat}&lon={lon}&units={units}&appid={key}",
		lat = location.lat,
		lon = location.lon,
		units = config.unit.to_param(),
		key = config.key,
	);

	let results = api_request(url)?;

	let weather = parse_weather(&config, results)?;

	Ok(weather)
}

pub fn config_to_weather(config: &Config) -> Result<WeatherResult> {
	let location = get_location(config)?;
	let weather = get_weather_at_location(config, location)?;

	Ok(weather)
}
