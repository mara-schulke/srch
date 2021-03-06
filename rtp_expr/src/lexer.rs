use std::iter::Peekable;

use crate::queries::Query;
use crate::logical_operators::Operator;


type Result<T> = std::result::Result<T, LexicalError>;

#[derive(Clone, Debug)]
pub enum LexicalError {
	UnknownSyntax,
	TrailingOperator,
	ToManyArguments
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

	pub fn next(&mut self) -> Result<Option<Token>> {
		let c = match self.peek() {
			Some(c) => c,
			None => return Ok(None),
		};

		self.iter.next();

		Ok(None)
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
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
