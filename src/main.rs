use std::io;

use clap::{Arg, ArgGroup, App, ValueHint};


const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


fn build_cli() -> App<'static> {
	fn build_subcommand(name: &'static str, descr: &'static str) -> App<'static> {
		App::new(name)
			.version(VERSION)
			.author(AUTHOR)
			.about(descr)
			.arg(Arg::new("mode")
				.short('m')
				.long("mode")
				.about("Sets the operation mode")
				.takes_value(true)
				.value_name("MODE")
				.default_value("line")
				.possible_values(&["line", "word"]))
			.arg(Arg::new("expression")
				.about("The rtp expression used to determine matches")
				.takes_value(true)
				.value_name("EXPRESSION")
				.value_hint(ValueHint::Other)
				.required(true)
				.index(1))
			.arg(Arg::new("input")
				.about("The path to the input file to use")
				.takes_value(true)
				.value_name("FILE")
				.value_hint(ValueHint::FilePath)
				.index(2))
			.arg(Arg::new("first")
				.short('f')
				.long("first")
				.conflicts_with("last")
				.about("Only use first match")
				.display_order(1))
			.arg(Arg::new("last")
				.short('l')
				.long("last")
				.conflicts_with("first")
				.about("Only use last match")
				.display_order(1))
			.group(ArgGroup::new("advanced")
				.arg("first")
				.arg("last"))
	}

	App::new(NAME)
		.version(VERSION)
		.author(AUTHOR)
		.about(DESCRIPTION)
		.subcommand(build_subcommand("filter", "Filter a text and print matches"))
		.subcommand(build_subcommand("ignore", "Ignore matches and print the rest"))
		.subcommand(build_subcommand("replace", "Replace matches with a given string"))
		// .subcommand(build_subcommand("exec", "Execute a given expression against a test string"))
}

fn main() -> io::Result<()> {
	let matches = build_cli().get_matches();

	match matches.subcommand() {
		Some(("filter", _subm)) => {
			println!("hello filter me");
		},
		Some(("ignore", _subm)) => {
			println!("hello ignore me");
		},
		Some(("replace", _subm)) => {
			println!("hello replace me");
		},
		_ => {}
	}

	Ok(())
}
