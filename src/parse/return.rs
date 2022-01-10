use crate::{check_token, parse::error::Error, types::TokenType};

use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::Return);
  ph.advance();

  let value = if let Some(TokenType::Integer(int)) = ph.peak(0) {
    let int = if *int >= 0 && *int < 256 {
      *int as u8
    } else {
      return Err(Error::new(&f!("Invalid return value: {int}"), ph.get(0)));
    };

    Node::Return(Some(int))
  } else {
    Node::Return(None)
  };

  ph.advance();

  check_token!(ph, TokenType::Semicolon);

  ph.advance();

  Ok(value)
}
