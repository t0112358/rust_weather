use clap::Parser;
use env_logger::Target;
use log::LevelFilter;
use rust_weather::cli::Command;
use std::process;

fn main() {
	let command = Command::parse();

	let filter_level = match command.verbose {
		true => LevelFilter::Debug,
		false => LevelFilter::Warn,
	};

	let name = env!("CARGO_PKG_NAME");
	env_logger::Builder::new()
		.filter_module(name, filter_level)
		.format_timestamp(None)
		.format_module_path(false)
		.format_target(false)
		.target(Target::Stdout)
		.init();

	if let Err(err) = command.run() {
		eprintln!("{}", err);
		process::exit(1);
	}

	process::exit(0);
}
