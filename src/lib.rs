use std::{error::Error, fs};

mod weather_api;

use weather_api::WeatherUnit;

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
			location: args[1].clone().replace(",", ""),
			key: key,
			unit: WeatherUnit::Imperial,
		})
	}
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
	let weather = weather_api::config_to_weather(config)?;

	println!("{}", weather);

	Ok(())
}
