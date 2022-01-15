use super::{ node::Node, parse_helper::ParseHelper};

use crate::{check_token, types::TokenType, parse::error::{ParserResult, Error}};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::LBracket);
  ph.advance();

  let mut values = Vec::new();
  loop {
    if let Some(token) = ph.get(0) {
      match token.r#type.clone() {
        TokenType::RBracket => break,
        TokenType::String(string) => values.push(string.clone()),
        _ => return Err(Error::unexpected(token))
      }
    } else {
      return Err(Error::end());
    }

    ph.advance();

    
    check_token!(ph, TokenType::Comma);
    ph.advance();
  }

  Ok(Node::Array(values))
}
