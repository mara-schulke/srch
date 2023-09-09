//! This crate contains a cli tool named `srch` for human readable text
//! searching. Typical tasks are filtering, ignoring or replacing words
//! or lines from input. The input can be either provided via stdin or be
//! read from an input file.
//!
//! # Usage
//!
//! `srch` is splitted into subcommands to make it as maintainable and
//! readable as possible. At the moment there are the following subcommands:
//! `for`, `not`, `replace`
//!
//! ## Modes
//! Before we can really dive in we need to take a quick look at the
//! available operation modes. Currently implemented are the two modes
//! `line` and `word`. As you might guess `line` is the default operation
//! mode for all commands. So everytime we execute a command which doesn't
//! specify another operation mode (namely `word`) the provided text
//! expression is executed on each line of the input, and each matched line
//! will be printed out.
//!
//! So to keep this short, here are two example which should clarify the modes
//!
//! ```bash
//! $ cat foo.txt
//! foo bar
//! foo baz
//! bar foo
//! bar baz
//! $ srch for 'start "foo"' foo.txt
//! foo bar
//! foo baz
//! $ srch for 'start "foo"' -m word foo.txt
//! foo
//! foo
//! foo
//! ```
//!
//! ## Filtering or Ignoring
//! The commands `for` and `not` work exactly the same. The only
//! difference is the inverted output is printed if `not` is used.
//!
//! `for` prints (depending on the mode) all lines or words matching
//! a specifc format described by the provided text expression. `ignore`
//! just prints everything except these matches.
//!
//! So the basic synopsis for these commands looks like this:
//!
//! ```bash
//! $ srch for <EXPRESSION> [FILE]
//! $ srch not <EXPRESSION> [FILE]
//! ```

use std::fs::File;
use std::io::{self, Read, Result};

use clap::{App, Arg, ArgGroup, ArgMatches, ValueHint};

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
        None => read_stdin()?,
    };

    let items = match matches.value_of("mode") {
        Some("line") => input.lines().map(|x| x.to_string()).collect(),
        Some("word") => input
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect(),
        Some(_) | None => vec![],
    };

    Ok(items)
}

// TODO: Add a "split" command to split text at certain chars

fn build_cli() -> App<'static> {
    fn build_subcommand(name: &'static str, descr: &'static str) -> App<'static> {
        App::new(name)
            .version(VERSION)
            .author(AUTHOR)
            .about(descr)
            .arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .help("Sets the operation mode")
                    .takes_value(true)
                    .value_name("MODE")
                    .default_value("line")
                    .possible_values(&["line", "word"]),
            )
            .arg(
                Arg::new("expression")
                    .help("The text expression used to determine matches")
                    .takes_value(true)
                    .value_name("EXPRESSION")
                    .value_hint(ValueHint::Other)
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("input")
                    .help("The path to the input file to use")
                    .takes_value(true)
                    .value_name("FILE")
                    .value_hint(ValueHint::FilePath)
                    .index(2),
            )
            .arg(
                Arg::new("first")
                    .short('f')
                    .long("first")
                    .conflicts_with_all(&["last"])
                    .help("Only use first match")
                    .display_order(1),
            )
            .arg(
                Arg::new("skip")
                    .short('s')
                    .long("skip")
                    .takes_value(true)
                    .value_name("n")
                    .value_hint(ValueHint::Other)
                    .conflicts_with_all(&["first", "last"])
                    .help("Skip n matches first match")
                    .display_order(1),
            )
            .arg(
                Arg::new("last")
                    .short('l')
                    .long("last")
                    .conflicts_with_all(&["first"])
                    .help("Only use last match")
                    .display_order(1),
            )
            .group(
                ArgGroup::new("advanced")
                    .arg("first")
                    .arg("last")
                    .arg("skip"),
            )
    }

    App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .subcommand(build_subcommand("for", "Filter a text and print matches"))
        .subcommand(build_subcommand("not", "Ignore matches and print the rest"))
        .subcommand(build_subcommand(
            "replace",
            "Replace matches with a given string",
        ))
    // .subcommand(build_subcommand("exec", "Execute a given expression against a test string"))
}

fn main() -> io::Result<()> {
    let matches = build_cli().get_matches();

    fn run_filter_command(submatches: &ArgMatches, invert_matches: bool) -> Result<()> {
        let expression = submatches.value_of("expression").unwrap_or_default();
        let input = read_input_from_matches(&submatches)?;

        let expr = match srch::Expression::new(&expression.to_owned()) {
            Ok(ast) => ast,
            Err(_) => {
                println!("Seems like you've provided an invalid text expression!");
                println!("Please head over to the text expression documentation:");
                println!("\nhttps://docs.rs/sel/");
                std::process::exit(1);
            }
        };

        let result = {
            let iter = input.iter();
            let filtered = iter.filter(|x| {
                let is_match = expr.matches(&x.to_string());

                match invert_matches {
                    true => !is_match,
                    false => is_match,
                }
            });

            filtered.map(|s| &**s).collect::<Vec<&str>>().join("\n")
        };

        if !result.is_empty() {
            println!("{}", result);
        }

        Ok(())
    }

    match matches.subcommand() {
        Some(("for", submatches)) => run_filter_command(submatches, false)?,
        Some(("not", submatches)) => run_filter_command(submatches, true)?,
        Some(("replace", _submatches)) => unimplemented!(),
        _ => {}
    }

    Ok(())
}
