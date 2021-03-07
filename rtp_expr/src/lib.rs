mod error;
mod lexer;
mod logical_operator;
mod parser;
mod query;
mod runtime;

use error::RTPExpressionResult;


pub use runtime::run;

pub fn into_ast(source: &String) -> RTPExpressionResult<parser::AST> {
	let tokens = lexer::lex(source)?;
	let ast = parser::parse(tokens)?;

	Ok(ast)
}
