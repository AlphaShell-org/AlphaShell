use crate::types::TokenType;

use super::{
  block::{self},
  error::{Error, Result},
  node::Node,
  parse_helper::ParseHelper,
  utils::{check_token, load_until_closing},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
  pub name: String,
  pub params: Vec<Node>,
  pub block: Box<Node>,
}

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  let params = Vec::new();

  ph.advance();

  let name = if let Some(token) = ph.peak(0) {
    if let TokenType::Identifier(name) = token {
      name.clone()
    } else {
      return Err(Error::unexpected(ph.get(0).unwrap()));
    }
  } else {
    return Err(Error::new("Unexpected end of input", ph.get(0)));
  };

  ph.advance();

  check_token(ph, &[TokenType::LParen])?;

  while let Some(param) = ph.get(0) {
    if let TokenType::Identifier(name) = token {
      params.push(name);
    } else {
      return Err(Error::unexpected(token));
    }
    
    
  }

  check_token(ph, &[TokenType::RParen])?;

  ph.advance();

  check_token(ph, &[TokenType::LBrace])?;

  let block = block::parse(ph)?;

  let node = Node::Function(Function {
    name,
    params,
    block: Box::new(block),
  });

  Ok(node)
}
