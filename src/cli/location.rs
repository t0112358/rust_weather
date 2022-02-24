use std::error::Error;

use crate::weather_api::{Client, Location};
use clap::Parser;

///Gets the geo-location of a provided string location
#[derive(Parser, Debug)]
pub struct LocationCommand {
	///The string location to retrieve geo-location of
	#[clap()]
	location: String,
}

impl LocationCommand {
	pub fn run(&self, key: &String) -> Result<(), Box<dyn Error>> {
		let client = Client::new().login(key.clone());

		let location = Location::from_string(&client, &self.location)?;

		println!("{}", location);

		Ok(())
	}
}
