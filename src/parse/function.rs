use crate::{check_token, types::TT};

use super::{
  block::{self},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
  name: String,
  params: Vec<String>,
  block: Box<Node>,
}

impl Function {
  pub fn new(name: String, params: Vec<String>, block: Box<Node>) -> Self {
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

  let name = match ph.peek(0) {
    Some(TT::Identifier(name)) => name.clone(),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  check_token!(ph, TT::LParen);

  ph.advance();

  loop {
    match ph.peek(0) {
      Some(TT::Identifier(name)) => params.push(name.clone()),
      Some(TT::RParen) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RParen) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  check_token!(ph, TT::RParen);

  ph.advance();

  let block = block::parse(ph)?;
  let node = Node::Function(Function::new(name, params, Box::new(block)));

  Ok(node)
}
