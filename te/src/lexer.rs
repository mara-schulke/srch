use std::iter::Peekable;

use crate::query::Query;
use crate::logical_operator::LogicalOperator;


type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
	UnknownSyntax,
	InternalError,
	ToManyArguments,
	NoLeadingZeros,
	UnclosedString,
	ExpectedString,
	ExpectedInteger,
	ExpectedQuery,
	ExpectedOperator
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

	fn expect_keyword(&mut self) -> Result<String> {
		let mut keyword = String::new();

		while let Some(x) = self.iter.peek() {
			if x.is_ascii_whitespace() {
				break;
			}

			keyword.push(*x);
			self.iter.next();
		}

		Ok(keyword)
	}

	fn query_from_keyword(&mut self, keyword: &String) -> Result<Option<Query>> {
		match keyword.as_str() {
			"starts" => Ok(Some(Query::Starts(self.expect_string()?))),
			"ends" => Ok(Some(Query::Ends(self.expect_string()?))),
			"contains" => Ok(Some(Query::Contains(self.expect_string()?))),
			"equals" => Ok(Some(Query::Equals(self.expect_string()?))),
			"length" => Ok(Some(Query::Length(self.expect_integer()?))),
			"numeric" => Ok(Some(Query::Numeric)),
			"alpha" => Ok(Some(Query::Alpha)),
			"alphanumeric" => Ok(Some(Query::Alphanumeric)),
			"special" => Ok(Some(Query::Special)),
			_ => Ok(None)
		}
	}

	fn operator_from_keyword(&mut self, keyword: &String) -> Result<Option<LogicalOperator>> {
		match keyword.as_str() {
			"and" => Ok(Some(LogicalOperator::And)),
			"or" => Ok(Some(LogicalOperator::Or)),
			_ => Ok(None)
		}
	}

	pub fn next(&mut self) -> Result<Option<Token>> {
		match self.peek() {
			Some(_) => {},
			None => return Ok(None)
		};

		let keyword = self.expect_keyword()?;

		if let Some(query) = self.query_from_keyword(&keyword)? {
			return Ok(Some(Token::Query(query)));
		} else if let Some(operator) = self.operator_from_keyword(&keyword)? {
			return Ok(Some(Token::LogicalOperator(operator)));
		}

		Err(Error::InternalError)
	}
}

pub fn lex(expr: &String) -> Result<Vec<Token>> {
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
					pretty_assertions::assert_eq!(lex(&input.to_string()).unwrap(), expected);
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
				"and",
				vec![
					Token::LogicalOperator(LogicalOperator::And)
				]
			),
			or: (
				"or",
				vec![
					Token::LogicalOperator(LogicalOperator::Or)
				]
			),
			and_and: (
				"and and",
				vec![
					Token::LogicalOperator(LogicalOperator::And),
					Token::LogicalOperator(LogicalOperator::And)
				]
			),
			or_or: (
				"or or",
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
				"starts \"baz\" and ends \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Ends("bar".to_string()))
				]
			),
			starts_or_ends: (
				"starts \"baz\" or ends \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Ends("bar".to_string()))
				]
			),
			starts_and_contains: (
				"starts \"baz\" and contains \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Contains("bar".to_string()))
				]
			),
			starts_or_contains: (
				"starts \"baz\" or contains \"bar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Contains("bar".to_string()))
				]
			),
			starts_and_equals: (
				"starts \"baz\" and equals \"bazbar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Equals("bazbar".to_string()))
				]
			),
			starts_or_equals: (
				"starts \"baz\" or equals \"bazbar\"",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Equals("bazbar".to_string()))
				]
			),
			starts_and_length: (
				"starts \"baz\" and length 10",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(10))
				]
			),
			starts_or_length: (
				"starts \"baz\" or length 12130",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(12130))
				]
			),
			starts_and_numeric: (
				"starts \"baz\" and numeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Numeric)
				]
			),
			starts_or_numeric: (
				"starts \"baz\" or numeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Numeric)
				]
			),
			starts_and_alpha: (
				"starts \"baz\" and alpha",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Alpha)
				]
			),
			starts_or_alpha: (
				"starts \"baz\" or alpha",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha)
				]
			),
			starts_and_alphanumeric: (
				"starts \"baz\" and alphanumeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Alphanumeric)
				]
			),
			starts_or_alphanumeric: (
				"starts \"baz\" or alphanumeric",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alphanumeric)
				]
			),
			starts_and_special: (
				"starts \"baz\" and special",
				vec![
					Token::Query(Query::Starts("baz".to_string())),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Special)
				]
			),
			starts_or_special: (
				"starts \"baz\" or special",
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
				"starts \"baz\" and ends \"bar\" or length 123 or special",
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
				"    numeric or alpha",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			begins_with_multiple_whitespaces_and_query_with_string: (
				"    starts \"foo\" or alpha",
				vec![
					Token::Query(Query::Starts("foo".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			begins_with_multiple_whitespaces_and_query_with_integer: (
				"    length 0 or alpha",
				vec![
					Token::Query(Query::Length(0)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			ends_with_multiple_whitespaces: (
				"numeric or alpha   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			ends_with_multiple_whitespacess_and_query_with_string: (
				"numeric or starts \"foo\"   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Starts("foo".to_string())),
				]
			),
			ends_with_multiple_whitespacess_and_query_with_integer: (
				"numeric or length 0   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(0)),
				]
			),
			starts_and_ends_with_multiple_whitespaces: (
				"   numeric or alpha   ",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_and_operator: (
				"numeric      or      alpha",
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_with_string_and_operator: (
				"starts \"foo\"      or      alpha",
				vec![
					Token::Query(Query::Starts("foo".to_string())),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
			has_multiple_whitespaces_between_query_with_integer_and_operator: (
				"length 999      or      alpha",
				vec![
					Token::Query(Query::Length(999)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
				]
			),
		}
	}
}
