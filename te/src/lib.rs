//! This crate provides a library for parsing, compiling, and executing text expressions.
//! The text expression syntax is kind of limited if compared to regular expressions, but
//! maintains a high readability in exchange. It's not the goal of the text expression
//! language to replace regular expression – it's meant to fill the lack of readability
//! for simple tasks
//! 
//! The text expression language was created in combination with the ter cli, a text
//! expression runner to make the execution and common usecases of text expressions as
//! easy as possible
//! 
//! This crate's documentation provides some simple examples, describes the
//! [supported syntax](#syntax) exhaustively.
//! 
//! For more specific details on text expressions, please see the documentation for the
//! [`TextExpression`](struct.TextExpression.html) struct.
//! 
//! # Usage
//! 
//! This crate is [on crates.io](https://crates.io/crates/te) and can be
//! used by adding `te` to your dependencies in your project's `Cargo.toml`.
//! 
//! ```toml
//! [dependencies]
//! te = "0.1"
//! ```
//! 
//! # Examples
//! 
//! ## 5 digit numbers
//! 
//! ```rust
//! use te::TextExpression;
//! let expr = TextExpression::new(&"numeric and length 5".to_owned()).unwrap();
//! assert!(expr.is_match_str("12345"));
//! ```
//! 
//! ## Naive email addresses
//! 
//! ```rust
//! use te::TextExpression;
//! let expr = TextExpression::new(&"contains \"@\" and contains \".com\"".to_owned()).unwrap();
//! assert!(expr.is_match_str("foo@baz.com"));
//! ```
//! 
//! ## Compiling a text expression only once
//! This is same problem as with regular expressions. It is an anti-pattern to
//! compile the same text expression in a loop since compilation is expensive.
//! It's recommended to use the [`lazy_static`](https://crates.io/crates/lazy_static)
//! crate to ensure that text expressions are compiled exactly once.
//! 
//! For example:
//! ```rust
//! use lazy_static::lazy_static;
//! use te::TextExpression;
//! 
//! fn utility(text: &str) -> bool {
//! 	lazy_static! {
//! 		static ref TE: TextExpression = TextExpression::new(&"...".to_owned()).unwrap();
//! 	}
//! 
//! 	TE.is_match(text)
//! }
//! 
//! fn main() {}
//! ```
//! 
//! Since this is a common problem this crate optionally exposes a macro for this:
//! 
//! ```rust
//! let te = lazy_text_expr!("length 5");
//! ```
//! 
//! To use this the `lazy` feature must be enabled in your `Cargo.toml`.
//! 
//! ```toml
//! [dependencies]
//! te = { version = "0.1", features = ["lazy"] }
//! ```
//! 
//! So know we can simplify the code from the first lazy example to use `lazy_text_expr!`:
//! 
//! ```rust
//! use te::lazy_text_exr;
//! 
//! fn utility(text: &str) -> bool {
//! 	lazy_text_expr!("...").is_match(text)
//! }
//! 
//! fn main() {}
//! ```
//! 
//! A lot cleaner, right? :) So now we know how we can use performant reusable text expressions!

mod error;
mod lexer;
mod logical_operator;
mod parser;
mod query;
mod runtime;

use error::TextExpressionResult;
use runtime::Runtime;


fn into_ast(source: &String) -> TextExpressionResult<parser::AST> {
	let tokens = lexer::lex(source)?;
	let ast = parser::parse(tokens)?;

	Ok(ast)
}

pub struct TextExpression {
	runtime: Runtime
}

impl TextExpression {

	pub fn new(source: &String) -> TextExpressionResult<Self> {
		let ast = into_ast(source.into())?;
		let runtime = Runtime::new(ast);

		Ok(Self {
			runtime
		})
	}

	pub fn is_match(&self, input: &String) -> bool {
		self.runtime.run(input)
	}

	pub fn is_match_str(&self, input: &str) -> bool {
		self.runtime.run(&input.to_string())
	}

}

#[cfg(feature = "lazy")]
#[macro_export]
macro_rules! lazy_text_expr {
	() => {
		
	};
}
