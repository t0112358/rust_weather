use std::error::Error;

use crate::weather_api::{Client, WeatherUnit};
use clap::Parser;

///Gets the weather at the provided location
#[derive(Parser, Debug)]
pub struct WeatherCommand {
	///The location to retrieve weather of
	#[clap()]
	location: String,

	///The weather unit to be used, "imperial" or "metric"
	#[clap(short, long, default_value = "imperial")]
	unit: WeatherUnit,
}

impl WeatherCommand {
	pub fn run(&self, key: &String) -> Result<(), Box<dyn Error>> {
		let client = Client::new()
			.with_unit(self.unit.clone())
			.login(key.clone());

		let weather_result = client.get_weather_at_string_loc(&self.location)?;

		println!("{}", weather_result.generate_weather_report());

		Ok(())
	}
}
