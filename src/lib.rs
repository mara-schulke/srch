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
//! # Examples
//!
//! ## 5 digit numbers
//!
//! ```rust
//! let expr = srch::Expression::new(&"numeric and length 5".to_owned()).unwrap();
//! assert!(expr.matches("12345"));
//! ```
//!
//! ## Naive email addresses
//!
//! ```rust
//! let expr = srch::Expression::new(&"contains \"@\" and contains \".com\"".to_owned()).unwrap();
//! assert!(expr.matches("foo@baz.com"));

// ```
//
// ## Compiling a text expression only once
// This is same problem as with regular expressions. It is an anti-pattern to
// compile the same text expression in a loop since compilation is expensive.
// It's recommended to use the [`lazy_static`](https://crates.io/crates/lazy_static)
// crate to ensure that text expressions are compiled exactly once.
//
// For example:
// ```rust
// use lazy_static::lazy_static;
// use srch::TextExpression;
//
// fn utility(text: &str) -> bool {
// 	lazy_static! {
// 		static ref TE: TextExpression = TextExpression::new(&"...".to_owned()).unwrap();
// 	}
//
// 	TE.is_match(text)
// }
//
// fn main() {}
// ```
//
// Since this is a common problem this crate optionally exposes a macro for this:
//
// ```rust
// let te = lazy_text_expr!("length 5");
// ```
//
// To use this the `lazy` feature must be enabled in your `Cargo.toml`.
//
// ```toml
// [dependencies]
// srch = { version = "0.1", features = ["lazy"] }
// ```
//
// So know we can simplify the code from the first lazy example to use `lazy_text_expr!`:
//
// ```rust
// use srch::lazy_text_exr;
//
// fn utility(text: &str) -> bool {
// 	lazy_text_expr!("...").is_match(text)
// }
//
// fn main() {}
// ```
//
// A lot cleaner, right? :) So now we know how we can use performant reusable text expressions!

mod error;
mod lexer;
mod logical_operator;
mod parser;
mod query;
mod runtime;

pub use error::Result;
pub use runtime::Runtime;

pub fn into_ast(source: &String) -> Result<parser::AST> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(tokens)?;
    Ok(ast)
}

pub struct Expression {
    runtime: Runtime,
}

impl Expression {
    pub fn new(source: &String) -> Result<Self> {
        let ast = into_ast(source.into())?;
        let runtime = Runtime::new(ast);

        Ok(Self { runtime })
    }

    pub fn matches(&self, input: impl AsRef<str>) -> bool {
        self.runtime.run(input.as_ref())
    }
}
