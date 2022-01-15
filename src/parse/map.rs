use crate::{check_token, types::TokenType, parse::error::Error};

use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> { 
  check_token!(ph, TokenType::LBrace);
  ph.advance();

  let mut items = Vec::new();
  loop {
    if ph.peak(0) == Some(&TokenType::RBrace) { break; }
    let key, value;
    match next {
      Some(TokenType::String(key_) => key = key, 
     _ => return Err(Error::unexpected(ph.get(0)))
    };
    ph.advance();

    match ph.peak(0) {
    , Some(TokenType::Colon), Some(TokenType::String(value))) => items.push((key.clone(), value.clone())),
      _ => return Err(Error::unexpected(ph.get(0)))
    }

    ph.advance();
  
    check_token!(ph, TokenType::Comma);
    ph.advance();
  }

  Ok(Node::Map(items))
}
