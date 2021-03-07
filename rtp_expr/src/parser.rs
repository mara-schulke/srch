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
		}

		else if self.tokens.len() > 2 {
			let left = Box::new(
				AST::Query(
					Self::expect_query(self.tokens[0].clone())?
				)
			);
			let operator = Self::expect_operator(self.tokens[1].clone())?;
			let right = {
				let mut cloned_tokens = self.tokens.clone();

				cloned_tokens.remove(0);
				cloned_tokens.remove(0);

				if cloned_tokens.len() == 1 {
					Box::new(
						AST::Query(
							Self::expect_query(cloned_tokens[0].clone())?
						)
					)
				} else {
					Box::new(parse(cloned_tokens)?)
				}
			};

			return Ok(ASTNode::BinaryExpression {
				left,
				operator,
				right
			})
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

	#[test]
	fn it_parses_a_single_query() {
		assert_eq!(
			parse(vec![
				Token::Query(Query::Numeric)
			]).unwrap(),
			AST::Query(Query::Numeric)
		);
	}

	#[test]
	fn it_parses_a_binary_query() {
		assert_eq!(
			parse(vec![
				Token::Query(Query::Numeric),
				Token::LogicalOperator(LogicalOperator::And),
				Token::Query(Query::Length(1))
			]).unwrap(),
			AST::BinaryExpression {
				left: Box::new(ASTNode::Query(Query::Numeric)),
				operator: LogicalOperator::And,
				right: Box::new(ASTNode::Query(Query::Length(1))),
			}
		);
	}
}
