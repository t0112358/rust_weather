use std::error::Error;

use clap::Parser;

use self::location::LocationCommand;
use self::weather::WeatherCommand;

mod location;
mod weather;

///Uses open weather api's database to provide weather data via a CLI.
#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
	///The Open Weather Api Key, recommended to use the environment variable variant
	#[clap(
		long = "open_weather_api_key",
		env = "OPEN_WEATHER_API_KEY",
		hide_env_values = true
	)]
	pub key: String,

	///Enables verbose output
	#[clap(short, long, global = true)]
	pub verbose: bool,

	#[clap(subcommand)]
	pub command: Subcommand,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
	Weather(WeatherCommand),
	Location(LocationCommand),
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
	match &args.command {
		Subcommand::Weather(weather_command) => weather_command.run(args),
		Subcommand::Location(location_command) => location_command.run(args),
	}
}
