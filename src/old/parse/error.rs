use crate::tokenize::{Position, Token};
use std::fmt;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
  pub msg: String,
  pub token: Option<Token>,
}

impl Error {
  pub fn new(msg: &str, token: Option<&Token>) -> Self {
    Error {
      msg: msg.to_string(),
      token: token.map(Clone::clone),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Error { msg, token } = self;

    if let Some(token) = token {
      let Position(line, column) = token.2;
      let (line, column) = (line + 1, column + 1); // account for zero indexing

      write_f!(f, "ParserError: \"{msg}\" at position {line}:{column}")
    } else {
      write_f!(f, "ParserError: \"{msg}\"")
    }
  }
}
