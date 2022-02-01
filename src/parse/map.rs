use crate::{check_token, parse::error::Error, types::TokenType};

use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::LBrace);
  ph.advance();

  let mut items = Vec::new();
  loop {
    if ph.peek(0) == Some(&TokenType::RBrace) {
      break;
    }

    let key = match ph.peek(0) {
      Some(TokenType::String(key)) => key.clone(),
      Some(_) => return Err(Error::unexpected(ph.get(0).unwrap())),
      _ => return Err(Error::end()),
    };

    ph.advance();

    check_token!(ph, TokenType::Colon);

    ph.advance();

    let value = match ph.peek(0) {
      Some(TokenType::String(value)) => value.clone(),
      Some(_) => return Err(Error::unexpected(ph.get(0).unwrap())),
      _ => return Err(Error::end()),
    };

    ph.advance();

    items.push((key, value));

    match ph.peek(0) {
      Some(TokenType::Comma) => ph.advance(),
      Some(TokenType::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph.get(0).unwrap())),
      _ => return Err(Error::end()),
    };
  }

  ph.advance();

  Ok(Node::Map(items))
}
