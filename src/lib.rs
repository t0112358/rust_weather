use std::error::Error;

mod weather_api;

use clap::Parser;
use weather_api::WeatherUnit;

///Uses open weather api's database to provide weather data via a CLI.
#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
	///The location to retrieve weather of
	#[clap(parse(from_str = parse_location))]
	location: String,

	///The weather unit to be used, "imperial" or "metric"
	#[clap(short, long, default_value = "imperial")]
	unit: WeatherUnit,

	///Enables verbose output
	#[clap(short, long)]
	verbose: bool,

	///The Open Weather Api Key, recommended to use the environment variable variant
	#[clap(
		long = "open_weather_api_key",
		env = "OPEN_WEATHER_API_KEY",
		hide_env_values = true
	)]
	key: String,
}

fn parse_location(original: &str) -> String {
	original.replace(",", "")
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
	let weather = weather_api::args_to_weather(&args)?;

	println!("{}", weather);

	Ok(())
}
