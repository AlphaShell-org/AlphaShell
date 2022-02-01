use super::{node::Node, parse_helper::ParseHelper};

use crate::{
  check_token,
  parse::error::{Error, ParserResult},
  types::TokenType,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::LBracket);
  ph.advance();

  let mut values = Vec::new();

  if ph.peek(0) == Some(&TokenType::LBracket) {
    return Ok(Node::Array(values));
  }

  loop {
    match ph.peek(0) {
      Some(TokenType::String(value)) => values.push(value.clone()),
      Some(_) => return Err(Error::unexpected(ph.get(0).unwrap())),
      _ => return Err(Error::end()),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TokenType::Comma) => ph.advance(),
      Some(TokenType::RBracket) => break,
      Some(_) => return Err(Error::unexpected(ph.get(0).unwrap())),
      _ => return Err(Error::end()),
    };
  }

  Ok(Node::Array(values))
}
