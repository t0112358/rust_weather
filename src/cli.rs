use std::error::Error;

use clap::Parser;

use self::location::LocationCommand;
use self::weather::WeatherCommand;

mod location;
mod weather;

///Uses open weather api's database to provide weather data via a CLI.
#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Command {
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
	pub subcommand: Subcommand,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
	Weather(WeatherCommand),
	Location(LocationCommand),
}

impl Command {
	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		match &self.subcommand {
			Subcommand::Weather(subcommand) => subcommand.run(self),
			Subcommand::Location(subcommand) => subcommand.run(self),
		}
	}
}
