use clap::Parser;
use rust_weather::Args;
use std::process;

fn main() {
	let args = Args::parse();

	if let Err(err) = rust_weather::run(args) {
		eprintln!("{}", err);
		process::exit(1);
	}

	process::exit(0);
}
