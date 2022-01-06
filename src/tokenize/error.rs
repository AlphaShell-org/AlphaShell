use super::{Position, State};
use std::fmt;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
  pub msg: String,
  pub pos: Position,
}

impl Error {
  pub fn new(msg: &str, state: &State) -> Error {
    Error {
      msg: msg.to_string(),
      pos: state.to_pos(),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Error { msg, pos } = self;
    let Position(line, column) = pos;
    let (line, column) = (line + 1, column + 1); // account for zero indexing

    write_f!(f, "LexerError: \"{msg}\" at position {line}:{column}")
  }
}
