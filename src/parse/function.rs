use crate::{check_token, types::TokenType};

use super::{
  block::{self},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
  name: String,
  params: Vec<Node>,
  block: Box<Node>,
}

impl Function {
  pub fn new(name: String, params: Vec<Node>, block: Box<Node>) -> Self {
    Self {
      name,
      params,
      block,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  let mut params = Vec::new();

  ph.advance();

  let name = if let Some(token) = ph.peek(0) {
    if let TokenType::Identifier(name) = token {
      name.clone()
    } else {
      return Err(Error::unexpected(ph.get(0).unwrap()));
    }
  } else {
    return Err(Error::new("Unexpected end of input", ph.get(0)));
  };

  ph.advance();

  check_token!(ph, TokenType::LParen);

  ph.advance();

  while let Some(param) = ph.peek(0) {
    if let TokenType::Identifier(name) = param {
      params.push(Node::Identifier(name.clone()));
    } else if param == &TokenType::RParen {
      break;
    } else {
      return Err(Error::unexpected(ph.get(0).unwrap()));
    }

    ph.advance();

    if let Some(token) = ph.peek(0) {
      match token {
        TokenType::Comma => ph.advance(),
        TokenType::RParen => break,
        _ => return Err(Error::unexpected(ph.get(0).unwrap())),
      }
    } else {
      return Err(Error::end());
    }
  }

  check_token!(ph, TokenType::RParen);

  ph.advance();

  let block = block::parse(ph)?;

  let node = Node::Function(Function::new(name, params, Box::new(block)));

  Ok(node)
}
