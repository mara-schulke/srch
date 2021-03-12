use std::io::{self, Read, Result};
use std::fs::File;

use clap::{Arg, ArgGroup, ArgMatches, App, ValueHint};


const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn read_stdin() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_to_string(&mut buffer)?;

	Ok(buffer)
}

fn read_file(path: &str) -> Result<String> {
	let mut file = File::open(path)?;
	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	Ok(contents)
}

fn read_input_from_matches(matches: &ArgMatches) -> io::Result<Vec<String>> {
	let input = match matches.value_of("input") {
		Some(path) => read_file(path)?,
		None => read_stdin()?
	};

	let items = match matches.value_of("mode") {
		Some("line") => input.lines().map(|x| x.to_string()).collect(),
		Some("word") => input.split_ascii_whitespace().map(|x| x.to_string()).collect(),
		Some(_) | None => vec![]
	};

	Ok(items)
}

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
				.about("The text expression used to determine matches")
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
				.conflicts_with_all(&["last"])
				.about("Only use first match")
				.display_order(1))
			.arg(Arg::new("skip")
				.short('s')
				.long("skip")
				.takes_value(true)
				.value_name("n")
				.value_hint(ValueHint::Other)
				.conflicts_with_all(&["first", "last"])
				.about("Skip n matches first match")
				.display_order(1))
			.arg(Arg::new("last")
				.short('l')
				.long("last")
				.conflicts_with_all(&["first"])
				.about("Only use last match")
				.display_order(1))
			.group(ArgGroup::new("advanced")
				.arg("first")
				.arg("last")
				.arg("skip"))
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

	fn run_filter_command(submatches: &ArgMatches, invert_matches: bool) -> Result<()> {
		let expression = submatches.value_of("expression").unwrap_or_default();
		let input = read_input_from_matches(&submatches)?;

		let compiled_expr = match te::into_ast(&expression.to_owned()) {
			Ok(ast) => ast,
			Err(_) => { panic!("Help the user at this point..") }
		};

		let result = {
			let iter = input.iter();
			let filtered = iter.filter(|x| {
				let is_match = te::run(compiled_expr.clone(), &x.to_string());

				match invert_matches{
					true => !is_match,
					false => is_match,
				}
			});

			filtered
				.map(|s| &**s)
				.collect::<Vec<&str>>()
				.join("\n")
		};

		if !result.is_empty() {
			println!("{}", result);
		}

		Ok(())
	}

	match matches.subcommand() {
		Some(("filter", submatches)) => run_filter_command(submatches, false)?,
		Some(("ignore", submatches)) => run_filter_command(submatches, true)?,
		Some(("replace", _submatches)) => unimplemented!(),
		_ => {}
	}

	Ok(())
}
