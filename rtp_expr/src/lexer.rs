use std::iter::Peekable;

use crate::queries::Query;
use crate::logical_operators::Operator;


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
	LogicalOperator(Operator)
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

		loop {
			let x = self.peek();

			match x {
				Some(x) => {
					if x == ' ' {
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
							int.push(x);
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
			if *x == ' ' {
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
		match self.peek() {
			Some(_) => {},
			None => return Ok(None)
		};

		// Ok(None)
		Ok(Some(Token::Query(self.expect_query().unwrap())))
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
	use crate::queries::Query;
	use crate::logical_operators::Operator;

	macro_rules! lexer_tests {
		($($name:ident: $value:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let (input, expected) = $value;
					assert_eq!(lex(input.to_string()).unwrap(), expected);
				}
			)*
		}
	}
	
	mod parses_single_query {
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
}
