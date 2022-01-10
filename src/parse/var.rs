use crate::{check_token, types::TokenType};

use super::{
  error::{Error, ParserResult},
  expression,
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationType {
  Let,
  Export,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
  r#type: DeclarationType,
  name: Box<Node>,
  value: Box<Node>,
}

impl Declaration {
  pub fn new(r#type: DeclarationType, name: Box<Node>, value: Box<Node>) -> Self {
    Self {
      r#type,
      name,
      value,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::Let | TokenType::Export);

  let r#type = if let Some(token) = ph.get(0) {
    match token.r#type {
      TokenType::Let => DeclarationType::Let,
      TokenType::Export => DeclarationType::Export,
      _ => return Err(Error::unexpected(token)),
    }
  } else {
    return Err(Error::end());
  };

  ph.advance();

  let r#name = if let Some(token) = ph.get(0) {
    match token.r#type.clone() {
      TokenType::Identifier(name) => Node::Identifier(name),
      _ => return Err(Error::unexpected(token)),
    }
  } else {
    return Err(Error::end());
  };

  ph.advance();

  check_token!(ph, TokenType::Assignment);

  ph.advance();

  let value = expression::parse(ph)?;

  check_token!(ph, TokenType::Semicolon);
  ph.advance();

  let declaration = Node::Declaration(Declaration::new(r#type, Box::new(name), Box::new(value)));

  Ok(declaration)
}
