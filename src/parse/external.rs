use anyhow::Result;

use super::parse_helper::ParseHelper;
use crate::{
  check_token,
  parse::{error, node::Node},
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  check_token!(ph, TT::External);

  ph.advance();

  match ph.peek(0) {
    Some(TT::Identifier(string) | TT::String(string)) => ph.variables.insert(string.clone()),
    Some(_) => return Err(error::unexpected(ph)),
    None => return Err(error::end(ph)),
  };

  loop {
    ph.advance();

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::Semicolon) => break,
      Some(_) => return Err(error::unexpected(ph)),
      None => return Err(error::end(ph)),
    };

    match ph.peek(0) {
      Some(TT::Identifier(string) | TT::String(string)) => ph.variables.insert(string.clone()),
      Some(_) => return Err(error::unexpected(ph)),
      None => return Err(error::end(ph)),
    };
  }

  ph.advance();

  Ok(Node::Empty)
}
