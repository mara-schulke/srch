use crate::{lexer, parser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LexicalError(lexer::Error),
    ParserError(parser::Error),
}

impl From<lexer::Error> for Error {
    fn from(err: lexer::Error) -> Self {
        Error::LexicalError(err)
    }
}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Self {
        Error::ParserError(err)
    }
}
