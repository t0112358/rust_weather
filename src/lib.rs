use std::error::Error;

mod weather_api;

use structopt::StructOpt;
use weather_api::WeatherUnit;

#[derive(StructOpt, Debug)]
#[structopt(about = "Uses open weather api's database to provide weather data via a CLI.")]
pub struct Args {
	#[structopt(parse(from_str = parse_location))]
	location: String,
	#[structopt(short, long, default_value = "imperial")]
	unit: WeatherUnit,
	#[structopt(short, long)]
	verbose: bool,
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
