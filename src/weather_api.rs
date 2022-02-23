use std::{
	error::Error,
	fmt::{self, Display},
	str::FromStr,
};

use reqwest::{blocking as reqwest_blocking, IntoUrl};

use serde_json::Value;

use log::debug;

const NO_KEY_PROVIDED_PANIC: &str =
	"Key was not provided to the client attempting to make an authenticated request.";
const NO_UNIT_PROVIDED_PANIC: &str =
	"Unit was not provided to the client attempting to make a request which requires a unit.";

#[derive(Debug)]
pub struct Location {
	lat: f64,
	lon: f64,
}

impl Location {
	pub fn from_string(client: &Client, string_loc: &String) -> Result<Location> {
		let key = client.key.as_ref().expect(NO_KEY_PROVIDED_PANIC);

		let url = format!(
			"https://api.openweathermap.org/geo/1.0/direct?q={location}&limit={limit}&appid={key}",
			location = string_loc,
			limit = 1,
			key = key,
		);

		debug!("Requesting location of \"{}\"", string_loc);

		let results = api_request(url)?;

		debug!("Parsing location of \"{}\"", string_loc);

		let location = parse_location(results)?;

		debug!("Got location of \"{}\": {}", string_loc, location);

		Ok(location)
	}
}

impl Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.lat, self.lon)
	}
}

#[derive(Debug, Clone)]
pub enum WeatherUnit {
	Imperial,
	Metric,
}

pub type Result<T> = std::result::Result<T, WeatherResultError>;

impl WeatherUnit {
	pub fn to_param(&self) -> String {
		self.to_string().to_lowercase()
	}
	pub fn to_temp_unit(&self) -> &str {
		match self {
			WeatherUnit::Imperial => "°F",
			WeatherUnit::Metric => "°C",
		}
	}
	pub fn to_pressure_unit(&self) -> &str {
		match self {
			WeatherUnit::Imperial => "hPa",
			WeatherUnit::Metric => "hPa",
		}
	}

	pub fn get_options() -> &'static str {
		"[`imperial`, `i`, `metric`, `m`]"
	}
}

impl FromStr for WeatherUnit {
	type Err = WeatherResultError;
	fn from_str<'a>(s: &'a str) -> Result<WeatherUnit> {
		match s {
			"imperial" | "i" => Ok(WeatherUnit::Imperial),
			"metric" | "m" => Ok(WeatherUnit::Metric),
			other => Err(WeatherResultError::InvalidWeatherUnit {
				given: other.to_string(),
			}),
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
	pub fn generate_weather_report(&self) -> String {
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
	///Something went wrong with the API request
	APIError {
		code: i64,
		message: String,
	},
	///No result was found
	NoResult,
	///Required fields were missing
	MissingFields,
	//Failed to parse the given weather unit
	InvalidWeatherUnit {
		given: String,
	},
}

impl Error for WeatherResultError {}

impl fmt::Display for WeatherResultError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::RequestError(err) => {
				write!(f, "Something went wrong getting the request: {}", err)
			}
			Self::APIError { code, message } => {
				write!(f, "API returned error code {code}: {message}")
			}
			Self::NoResult => write!(f, "No result was found"),
			Self::MissingFields => write!(f, "Required fields were missing"),
			Self::InvalidWeatherUnit { given } => {
				write!(
					f,
					"`{}`. Possible values: {}",
					given,
					WeatherUnit::get_options()
				)
			}
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

	if let Some(error) = result.as_object() {
		let code = error.get("cod");
		let message = error.get("message");
		if code.is_some() {
			if message.is_some() {
				let code = code.unwrap().as_i64();
				let message = message.unwrap().as_str();
				if code.is_some() && message.is_some() {
					return Err(WeatherResultError::APIError {
						code: code.unwrap(),
						message: message.unwrap().to_string(),
					});
				}
			} else {
				if let Some(code) = code.unwrap().as_i64() {
					if code >= 400 {
						let message = match code {
							404 => "Result not found",
							_ => "Unknown error",
						}
						.to_string();

						return Err(WeatherResultError::APIError { code, message });
					}
				}
			}
		}
	}

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

fn parse_weather(value: Value, unit: &WeatherUnit) -> Result<WeatherResult> {
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
		unit: unit.clone(),
	})
}

pub struct Client {
	key: Option<String>,
	unit: Option<WeatherUnit>,
}

impl Client {
	pub fn new() -> Client {
		Client {
			key: None,
			unit: None,
		}
	}

	pub fn login(mut self, key: String) -> Client {
		self.key = Some(key);
		self
	}

	pub fn with_unit(mut self, unit: WeatherUnit) -> Client {
		self.unit = Some(unit);
		self
	}

	pub fn get_weather_at_location(&self, location: Location) -> Result<WeatherResult> {
		let unit = self.unit.as_ref().expect(NO_UNIT_PROVIDED_PANIC);
		let key = self.key.as_ref().expect(NO_KEY_PROVIDED_PANIC);

		let url = format!(
			"https://api.openweathermap.org/data/2.5/weather?\
			lat={lat}&lon={lon}&units={units}&appid={key}",
			lat = location.lat,
			lon = location.lon,
			units = unit.to_param(),
			key = key,
		);

		debug!("Requesting weather at {}", location);

		let results = api_request(url)?;

		debug!("Parsing weather at {}", location);

		let weather = parse_weather(results, &unit)?;

		debug!("Got weather at {}", location);

		Ok(weather)
	}

	pub fn get_weather_at_string_loc(&self, string_loc: &String) -> Result<WeatherResult> {
		let location = Location::from_string(self, string_loc)?;
		let weather = self.get_weather_at_location(location)?;

		Ok(weather)
	}
}
