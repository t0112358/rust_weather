use rust_weather::Config;
use std::{env, process};

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("{}", err);
		process::exit(1);
	});

	if let Err(err) = rust_weather::run(&config) {
		println!("{}", err);
		process::exit(1);
	}

	process::exit(0);
}
