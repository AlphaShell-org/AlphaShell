use super::{
  general::{is_next_valid, Node},
  ParseHelper, Result, Token, TokenType,
};

struct Import {
  target: Vec<Token>,
}

impl Node for Import {}

pub fn parse_import(ph: &mut ParseHelper) -> Result<Import> {
  let target = Vec::new();

  loop {
    is_next_valid(ph, &[TokenType::Identifier, TokenType::String])?;

    target.push(ph.token().clone());

    is_next_valid(ph, &[TokenType::Comma, TokenType::Semicolon])?;

    ph.advance();

    if ph.peak(1) == Some(TokenType::Semicolon) {
      break;
    }
  }

  return Ok(Import { target });
}
