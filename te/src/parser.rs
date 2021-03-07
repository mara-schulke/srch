use crate::lexer::Token;
use crate::query::Query;
use crate::logical_operator::LogicalOperator;


type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
	ExpectedQuery,
	ExpectedOperator,
	EmptyExpression,
	InternalError
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
	Query(Query),
	BinaryExpression {
		left: Box<ASTNode>,
		operator: LogicalOperator,
		right: Box<ASTNode>,
	},
}

pub type AST = ASTNode;

#[derive(Clone, Debug)]
pub struct Parser {
	tokens: Vec<Token>
}

impl Parser {
	
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens
		}
	}

	fn expect_query(token: Token) -> Result<Query> {
		match token {
			Token::Query(q) => Ok(q),
			Token::LogicalOperator(_) => Err(Error::ExpectedQuery)
		}
	}

	fn expect_operator(token: Token) -> Result<LogicalOperator> {
		match token {
			Token::LogicalOperator(op) => Ok(op),
			Token::Query(_) => Err(Error::ExpectedOperator)
		}
	}

	fn validate_structure(&self) -> Result<()> {
		if self.tokens.is_empty() {
			return Err(Error::EmptyExpression);
		}

		match self.tokens.first() {
			Some(tkn) => { Self::expect_query(tkn.clone())?; },
			None => {}
		}

		match self.tokens.last() {
			Some(tkn) => { Self::expect_query(tkn.clone())?; },
			None => {}
		}

		let mut expect_query = true;

		for tkn in self.tokens.clone() {
			if expect_query {
				Self::expect_query(tkn)?;
			} else {
				Self::expect_operator(tkn)?;
			}

			expect_query = !expect_query;
		}

		Ok(())
	}

	pub fn parse(&mut self) -> Result<AST> {
		self.validate_structure()?;

		if self.tokens.len() == 1 {
			let query = Self::expect_query(self.tokens[0].clone())?;
			return Ok(AST::Query(query));
		} else if self.tokens.len() > 2 {
			let next_operator = Self::expect_operator(self.tokens[1].clone())?;

			if next_operator == LogicalOperator::And {
				let mut tkns = self.tokens.clone();

				if let Some(index) = tkns.iter().position(|tkn| tkn == &Token::LogicalOperator(LogicalOperator::Or)) {
					let right_tokens = tkns.split_off(index)[1..].to_vec();

					let left = Box::new(parse(tkns)?);
					let right = Box::new(parse(right_tokens)?);

					return Ok(ASTNode::BinaryExpression {
						left,
						operator: LogicalOperator::Or,
						right
					});
				}
			}

			let left = Box::new(
				AST::Query(
					Self::expect_query(self.tokens[0].clone())?
				)
			);
	
			let right = Box::new(parse(self.tokens.clone()[2..].to_vec())?);
	
			return Ok(ASTNode::BinaryExpression {
				left,
				operator: next_operator,
				right
			});
		}

		Err(Error::InternalError)
	}

}

pub fn parse(tokens: Vec<Token>) -> Result<AST> {
	let mut parser = Parser::new(tokens);

	parser.parse()
}


#[cfg(test)]
mod tests {
	use super::{parse, Parser, AST, ASTNode};
	use crate::lexer::Token;
	use crate::logical_operator::LogicalOperator;
	use crate::query::Query;

	macro_rules! parser_tests {
		($($name:ident: $value:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let (tokens, expected_ast) = $value;
					pretty_assertions::assert_eq!(parse(tokens).unwrap(), expected_ast);
				}
			)*
		}
	}

	mod it_parses_single_queries {
		use super::*;

		parser_tests! {
			numeric: (
				vec![
					Token::Query(Query::Numeric)
				],
				AST::Query(Query::Numeric)
			),
		}
	}

	mod it_parses_binary_queries {
		use super::*;

		parser_tests! {
			numeric_and_length: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(1))
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::And,
					right: Box::new(ASTNode::Query(Query::Length(1))),
				}
			),
			numeric_or_length: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(1))
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::Query(Query::Length(1))),
				}
			),
		}
	}

	mod it_parses_composed_queries {
		use super::*;

		parser_tests! {
			numeric_and_length_or_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(1)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::Query(Query::Numeric)),
						operator: LogicalOperator::And,
						right: Box::new(ASTNode::Query(Query::Length(1))),
					}),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::Query(Query::Special)),
				}
			),
			numeric_or_length_and_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(1)),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::Query(Query::Length(1))),
						operator: LogicalOperator::And,
						right: Box::new(ASTNode::Query(Query::Special)),
					}),
				}
			),
			numeric_and_length_and_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(1)),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::And,
					right: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::Query(Query::Length(1))),
						operator: LogicalOperator::And,
						right: Box::new(ASTNode::Query(Query::Special)),
					}),
				}
			),
			numeric_or_length_or_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Length(1)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::Query(Query::Length(1))),
						operator: LogicalOperator::Or,
						right: Box::new(ASTNode::Query(Query::Special)),
					}),
				}
			),
		}
	}

	mod it_parses_complex_queries_and_preserves_operator_precedence {
		use super::*;

		parser_tests! {
			numeric_or_alpha_or_alphanumeric_and_length_or_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alphanumeric),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(100)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::Query(Query::Alpha)),
						operator: LogicalOperator::Or,
						right: Box::new(ASTNode::BinaryExpression {
							left: Box::new(ASTNode::BinaryExpression {
								left: Box::new(ASTNode::Query(Query::Alphanumeric)),
								operator: LogicalOperator::And,
								right: Box::new(ASTNode::Query(Query::Length(100))),
							}),
							operator: LogicalOperator::Or,
							right: Box::new(ASTNode::Query(Query::Special))
						}),
					}),
				}
			),
			numeric_or_alpha_and_alphanumeric_and_length_or_special: (
				vec![
					Token::Query(Query::Numeric),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Alpha),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Alphanumeric),
					Token::LogicalOperator(LogicalOperator::And),
					Token::Query(Query::Length(100)),
					Token::LogicalOperator(LogicalOperator::Or),
					Token::Query(Query::Special),
				],
				AST::BinaryExpression {
					left: Box::new(ASTNode::Query(Query::Numeric)),
					operator: LogicalOperator::Or,
					right: Box::new(ASTNode::BinaryExpression {
						left: Box::new(ASTNode::BinaryExpression {
							left: Box::new(ASTNode::Query(Query::Alpha)),
							operator: LogicalOperator::And,
							right: Box::new(ASTNode::BinaryExpression {
								left: Box::new(ASTNode::Query(Query::Alphanumeric)),
								operator: LogicalOperator::And,
								right: Box::new(ASTNode::Query(Query::Length(100))),
							})
						}),
						operator: LogicalOperator::Or,
						right: Box::new(ASTNode::Query(Query::Special))
					}),
				}
			),
		}
	}
}
