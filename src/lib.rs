use std::error::Error;

mod weather_api;

use structopt::StructOpt;
use weather_api::WeatherUnit;

///Uses open weather api's database to provide weather data via a CLI.
#[derive(StructOpt, Debug)]
pub struct Args {
	///The location to retrieve weather of
	#[structopt(parse(from_str = parse_location))]
	location: String,

	///The weather unit to be used, "imperial" or "metric"
	#[structopt(short, long, default_value = "imperial")]
	unit: WeatherUnit,

	///Enables verbose output
	#[structopt(short, long)]
	verbose: bool,

	///The Open Weather Api Key, recommended to use the environment variable variant
	#[structopt(
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
