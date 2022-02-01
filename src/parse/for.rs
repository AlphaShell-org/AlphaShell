use crate::{check_token, types::TokenType};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct For {
  start: i64,
  end: i64,
  step: i64,
  variable: String,
  block: Box<Node>,
}

impl For {
  pub fn new(start: i64, end: i64, step: i64, variable: String, block: Box<Node>) -> Self {
    Self {
      start,
      end,
      step,
      variable,
      block,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Foreach {
  variable: String,
  iterable: Box<Node>,
}

impl Foreach {
  pub fn new(variable: String, iterable: Box<Node>) -> Self {
    Self { variable, iterable }
  }
}

fn parse_for(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  unimplemented!();
}

fn parse_foreach(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  unimplemented!();
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::For);

  ph.advance();

  let variable = match ph.peek(0) {
    Some(TokenType::String(variable_)) => variable_.clone(),
    _ => return Err(Error::unexpected(ph.get(0).unwrap())),
  };

  ph.advance();

  check_token!(ph, TokenType::In);

  ph.advance();

  let node = match ph.peek(0) {
    Some(TokenType::Integer(_)) => parse_for(ph, variable)?,
    Some(TokenType::At | TokenType::LBracket) => parse_foreach(ph, variable)?,
    _ => return Err(Error::unexpected(ph.get(0).unwrap())),
  };

  Ok(node)
}
