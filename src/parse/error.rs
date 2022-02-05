use crate::types::{Position, Token};
use std::fmt;

use super::parse_helper::ParseHelper;

pub type ParserResult<T> = Result<T, Error>;

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

  pub fn unexpected(ph: &ParseHelper) -> Self {
    let token = ph.get(0).unwrap();

    #[cfg(debug_assertions)]
    println!(
      "{}\ncurrent index: {}",
      ph.pretty_print_tokens(),
      ph.get_index()
    );

    Self::new(&f!("Unexpected token {token}"), Some(token))
  }

  pub fn end(ph: &ParseHelper) -> Error {
    #[cfg(debug_assertions)]
    println!(
      "{}\ncurrent index: {}",
      ph.pretty_print_tokens(),
      ph.get_index()
    );

    let last = ph.get_tokens().last().unwrap();

    Self::new(&f!("Unexpected end of input after {last}"), Some(last))
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Error { msg, token } = self;
    
    if let Some(token) = token {
      let Position(line, column) = token.position;
      let (line, column) = (line + 1, column + 1); // account for zero indexing

      write_f!(f, "ParserError: \"{msg}\" at position {line}:{column}")
    } else {
      write_f!(f, "ParserError: \"{msg}\"")
    }
  }
}
