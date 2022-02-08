use reqwest::{blocking as reqwest_blocking, IntoUrl};
use std::{
	error::Error,
	fmt::{self},
	fs,
};

#[derive(Clone)]
pub struct Config {
	location: String,
	key: String,
	unit: WeatherUnit,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() > 2 {
			return Err("Too many arguments");
		} else if args.len() < 2 {
			return Err("Too few arguments");
		}

		let key = fs::read_to_string("key.txt").expect("No key.txt file!");

		Ok(Config {
			location: args[1].clone(),
			key: key,
			unit: WeatherUnit::Imperial,
		})
	}
}

#[derive(Debug)]
struct Location {
	lat: f64,
	lon: f64,
}

#[derive(Debug, Clone)]
enum WeatherUnit {
	Imperial,
	Metric,
}

impl WeatherUnit {
	fn to_param(&self) -> &str {
		match self {
			WeatherUnit::Imperial => "imperial",
			WeatherUnit::Metric => "metric",
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
struct WeatherResult {
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
  Temperature: {}  Feels like: {}
  Min: {}  Max: {}
  Pressure: {}  Humidity: {}
  All units are in the {} system",
			self.temp,
			self.temp_feels_like,
			self.temp_min,
			self.temp_max,
			self.pressure,
			self.humidity,
			self.unit.to_param(),
		)
	}
}

impl fmt::Display for WeatherResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{}", self.generate_weather_report())
	}
}

#[derive(Debug)]
struct WeatherResultError {
	kind: WeatherResultErrorKind,
}

impl Error for WeatherResultError {}

impl fmt::Display for WeatherResultError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.kind)
	}
}

impl WeatherResultError {
	fn new(kind: WeatherResultErrorKind) -> WeatherResultError {
		return WeatherResultError { kind };
	}
}

#[derive(Debug)]
enum WeatherResultErrorKind {
	///No result was found
	NoResult,
	///Fields were missing
	MissingFields,
}

use serde_json::Value;
fn api_request<U: IntoUrl>(url: U) -> Result<Value, reqwest::Error> {
	let client = reqwest_blocking::Client::new();

	let result: Value = client.get(url).send()?.json()?;

	Ok(result)
}

fn parse_location(value: Value) -> Result<Location, WeatherResultError> {
	let results = match value {
		Value::Array(array) => array,
		_ => return Err(WeatherResultError::new(WeatherResultErrorKind::NoResult)),
	};

	let result = match results.get(0) {
		Some(Value::Object(object)) => object,
		_ => return Err(WeatherResultError::new(WeatherResultErrorKind::NoResult)),
	};

	let lat = result.get("lat");
	let long = result.get("lon");

	if lat.is_none() || long.is_none() {
		return Err(WeatherResultError::new(
			WeatherResultErrorKind::MissingFields,
		));
	}

	Ok(Location {
		lat: lat.unwrap().as_f64().unwrap(),
		lon: long.unwrap().as_f64().unwrap(),
	})
}

fn get_location(config: &Config) -> Result<Location, Box<dyn Error>> {
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

fn parse_weather(config: &Config, value: Value) -> Result<WeatherResult, WeatherResultError> {
	let result = match value {
		Value::Object(object) => object,
		_ => return Err(WeatherResultError::new(WeatherResultErrorKind::NoResult)),
	};

	let main = match result.get("main") {
		Some(Value::Object(object)) => object,
		_ => {
			return Err(WeatherResultError::new(
				WeatherResultErrorKind::MissingFields,
			))
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

fn get_weather_at_location(
	config: &Config,
	location: Location,
) -> Result<WeatherResult, Box<dyn Error>> {
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

fn config_to_weather(config: &Config) -> Result<WeatherResult, Box<dyn Error>> {
	let location = get_location(config)?;
	let weather = get_weather_at_location(config, location)?;

	Ok(weather)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
	let weather = config_to_weather(config)?;

	println!("{}", weather);

	Ok(())
}
