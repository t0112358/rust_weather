use rust_weather::Args;
use std::process;
use structopt::StructOpt;

fn main() {
	let args = Args::from_args();

	if let Err(err) = rust_weather::run(args) {
		eprintln!("{}", err);
		process::exit(1);
	}

	process::exit(0);
}
