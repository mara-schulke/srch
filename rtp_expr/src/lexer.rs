use std::iter::Peekable;

use crate::query::Query;
use crate::logical_operator::LogicalOperator;


type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
	UnknownSyntax,
	InternalError,
	TrailingOperator,
	ToManyArguments,
	NoLeadingZeros,
	UnclosedString,
	ExpectedString,
	ExpectedInteger,
	ExpectedQuery
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Query(Query),
	LogicalOperator(LogicalOperator)
}

#[derive(Clone)]
pub struct Lexer<I: Iterator<Item = char> + Clone> {
	iter: Peekable<I>,
}

impl<I: Iterator<Item = char> + Clone> Lexer<I> {

	fn new(input: I) -> Self {
		Self {
			iter: input.peekable(),
		}
	}

	fn trim(&mut self) {
		loop {
			match self.iter.peek().cloned() {
				Some(c) if c.is_ascii_whitespace() => {
					self.iter.next();
				}
				_ => break,
			}
		}
	}

	fn peek(&mut self) -> Option<char> {
		self.trim();
		self.iter.peek().cloned()
	}

	fn read_string(&mut self) -> Result<Option<String>> {
		match self.peek() {
			Some('"') => {},
			Some(_) => return Ok(None),
			None => return Ok(None)
		};

		// skip opening quote
		self.iter.next();

		let mut seq = String::new();

		// read string contents
		// using self.iter.peek since it doesnt strip whitespaces like self.peek
		loop {
			let x = self.iter.peek();
			match x {
				Some(x) => {
					if *x == '"' {
						break;
					}

					seq.push(*x);
					self.iter.next();
				}
				None => return Err(Error::UnclosedString),
			}
		}

		// skip closing quote
		self.iter.next();

		Ok(Some(seq))
	}

	fn expect_string(&mut self) -> Result<String> {
		match self.read_string()? {
			Some(s) => Ok(s),
			None => Err(Error::ExpectedString)
		}
	}

	fn read_integer(&mut self) -> Result<Option<u64>> {
		let mut int = String::new();

		self.trim();

		loop {
			let x = self.iter.peek();

			match x {
				Some(x) => {
					println!("{:#?}", *x);

					if x.is_ascii_whitespace() {
						break;
					}

					if !x.is_ascii_digit() {
						return Err(Error::ExpectedInteger);
					}

					match int.as_str() {
						"0" => {
							return Err(Error::NoLeadingZeros);
						},
						_ => {
							int.push(*x);
							self.iter.next();
						}
					}
				}
				None => match int.as_str() {
					"" => return Err(Error::ExpectedInteger),
					_ => { break; }
				}
			}
		}

		match int.parse::<u64>() {
			Ok(parsed) => Ok(Some(parsed)),
			Err(_) => Err(Error::InternalError)
		}
	}

	fn expect_integer(&mut self) -> Result<u64> {
		match self.read_integer()? {
			Some(i) => Ok(i),
			None => Err(Error::ExpectedInteger)
		}
	}

	fn expect_query(&mut self) -> Result<Query> {
		let mut query_name = String::new();

		while let Some(x) = self.iter.peek() {
			if x.is_ascii_whitespace() {
				break;
			}

			query_name.push(*x);
			self.iter.next();
		}

		match query_name.as_str() {
			"starts" => Ok(Query::Starts(self.expect_string()?)),
			"ends" => Ok(Query::Ends(self.expect_string()?)),
			"contains" => Ok(Query::Contains(self.expect_string()?)),
			"equals" => Ok(Query::Equals(self.expect_string()?)),
			"length" => Ok(Query::Length(self.expect_integer()?)),
			"numeric" => Ok(Query::Numeric),
			"alpha" => Ok(Query::Alpha),
			"alphanumeric" => Ok(Query::Alphanumeric),
			"special" => Ok(Query::Special),
			_ => Err(Error::ExpectedQuery)
		}
	}

	pub fn next(&mut self) -> Result<Option<Token>> {
		let c = match self.peek() {
			Some(c) => c,
			None => return Ok(None)
		};

		let token = match c {
			'&' => {
				self.iter.next();
				Token::LogicalOperator(LogicalOperator::And)
			},
			'|' => {
				self.iter.next();
				Token::LogicalOperator(LogicalOperator::Or)
			},
			_ => Token::Query(self.expect_query().unwrap())
		};

		Ok(Some(token))
	}
}

pub fn lex(expr: String) -> Result<Vec<Token>> {
	let mut lexer = Lexer::new(expr.chars());
	let mut tokens: Vec<Token> = Vec::new();

	while let Some(token) = lexer.next()? {
		tokens.push(token);
	}

	Ok(tokens)
}


#[cfg(test)]
mod tests {
	use super::{lex, Token};
	use crate::query::Query;
	use crate::logical_operator::LogicalOperator;

	macro_rules! lexer_tests {
		($($name:ident: $value:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let (input, expected) = $value;
					pretty_assertions::assert_eq!(lex(input.to_string()).unwrap(), expected);
				}
			)*
		}
	}
	
	mod it_parses_a_single_query {
		use super::*;

		lexer_tests! {
			starts: (
				"starts \"foo\"",
				vec![
					Token::Query(Query::Starts("foo".to_string()))
				]
			),
			ends: (
				"ends \"foo\"",
				vec![
					Token::Query(Query::Ends("foo".to_string()))
				]
			),
			contains: (
				"contains \"foo\"",
				vec![
					Token::Query(Query::Contains("foo".to_string()))
				]
			),
			equals: (
				"equals \"foo\"",
				vec![
					Token::Query(Query::Equals("foo".to_string()))
				]
			),
			length: (
				"length 10",
				vec![
					Token::Query(Query::Length(10))
				]
			),
			numeric: (
				"numeric",
				vec![
					Token::Query(Query::Numeric)
				]
			),
			alpha: (
				"alpha",
				vec![
					Token::Query(Query::Alpha)
				]
			),
			alphanumeric: (
				"alphanumeric",
				vec![
					Token::Query(Query::Alphanumeric)
				]
			),
			special: (
				"special",
				vec![
					Token::Query(Query::Special)
				]
			),
		}
	}

	mod it_parses_operators {
		use super::*;

		//todo: add more tests which handle invalid operators

		lexer_tests! {
			and: (
				"&",
				vec![
					Token::LogicalOperator(LogicalOperator::And)
				]
			),
			or: (
				"|",
				vec![
					Token::LogicalOperator(LogicalOperator::Or)
				]
			),
			and_and: (
				"& &",
				vec![
					Token::LogicalOperator(LogicalOperator::And),
					Token::LogicalOperator(LogicalOperator::And)
				]
			),
			or_or: (
				"| |",
				vec![
					Token::LogicalOperator(LogicalOperator::Or),
					Token::LogicalOperator(LogicalOperator::Or)
				]
			),
		}
	}

	mod it_parses_dual_expressions {
		use super::*;

		lexer_tests! {
			starts_and_ends: (
				"starts \"baz\" & ends \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Ends("bar".to_string()))
				]
			),
			starts_or_ends: (
				"starts \"baz\" | ends \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Ends("bar".to_string()))
				]
			),
			starts_and_contains: (
				"starts \"baz\" & contains \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Contains("bar".to_string()))
				]
			),
			starts_or_contains: (
				"starts \"baz\" | contains \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Contains("bar".to_string()))
				]
			),
			starts_and_equals: (
				"starts \"baz\" & equals \"bazbar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Equals("bazbar".to_string()))
				]
			),
			starts_or_equals: (
				"starts \"baz\" | equals \"bazbar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Equals("bazbar".to_string()))
				]
			),
			starts_and_length: (
				"starts \"baz\" & length 10",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(10))
				]
			),
			starts_or_length: (
				"starts \"baz\" | length 12130",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(12130))
				]
			),
			starts_and_numeric: (
				"starts \"baz\" & numeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Numeric)
				]
			),
			starts_or_numeric: (
				"starts \"baz\" | numeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Numeric)
				]
			),
			starts_and_alpha: (
				"starts \"baz\" & alpha",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Alpha)
				]
			),
			starts_or_alpha: (
				"starts \"baz\" | alpha",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha)
				]
			),
			starts_and_alphanumeric: (
				"starts \"baz\" & alphanumeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Alphanumeric)
				]
			),
			starts_or_alphanumeric: (
				"starts \"baz\" | alphanumeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alphanumeric)
				]
			),
			starts_and_special: (
				"starts \"baz\" & special",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Special)
				]
			),
			starts_or_special: (
				"starts \"baz\" | special",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special)
				]
			),
		}
	}

	mod it_parses_complex_expressions {
		use super::*;

		lexer_tests! {
			starts_and_ends_or_length_or_special: (
				"starts \"baz\" & ends \"bar\" | length 123 | special",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Ends("bar".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(123)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special),
				]
			),
		}
	}

	mod it_ignores_multiple_or_trailing_whitespaces {
		use super::*;

		lexer_tests! {
			begins_with_multiple_whitespaces: (
				"    numeric | alpha",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			begins_with_multiple_whitespaces_and_query_with_string: (
				"    starts \"foo\" | alpha",
				vec![
					Token::Query(Query::Starts("foo".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			begins_with_multiple_whitespaces_and_query_with_integer: (
				"    length 0 | alpha",
				vec![
					Token::Query(Query::Length(0)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			ends_with_multiple_whitespaces: (
				"numeric | alpha   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			ends_with_multiple_whitespacess_and_query_with_string: (
				"numeric | starts \"foo\"   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Starts("foo".to_string())),
				]
			),
			ends_with_multiple_whitespacess_and_query_with_integer: (
				"numeric | length 0   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(0)),
				]
			),
			starts_and_ends_with_multiple_whitespaces: (
				"   numeric | alpha   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_and_operator: (
				"numeric      |      alpha",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_with_string_and_operator: (
				"starts \"foo\"      |      alpha",
				vec![
					Token::Query(Query::Starts("foo".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_with_integer_and_operator: (
				"length 999      |      alpha",
				vec![
					Token::Query(Query::Length(999)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
		}
	}
}
