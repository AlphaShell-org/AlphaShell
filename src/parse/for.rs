use super::{
  block::{self, Block},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value::{self, Value},
};
use crate::{check_token, types::TT};

#[derive(Debug, PartialEq, Clone)]
pub struct For {
  pub start: i32,
  pub end: i32,
  pub step: i32,
  pub variable: String,
  pub block: Box<Node>,
}

impl For {
  pub fn new(start: i32, end: i32, step: i32, variable: String, block: Box<Node>) -> Self {
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
  pub variable: String,
  pub iterable: Value,
  pub block: Block,
}

impl Foreach {
  pub fn new(iterable: Value, variable: String, block: Block) -> Self {
    Self {
      variable,
      iterable,
      block,
    }
  }
}

fn parse_for(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  let start = match ph.peek(0) {
    Some(TT::Integer(num)) => *num,
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();
  check_token!(ph, TT::Range);
  ph.advance();

  let end = match ph.peek(0) {
    Some(TT::Integer(num)) => *num,
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  let step = if let Some(TT::Range) = ph.peek(0) {
    ph.advance();

    let step = match ph.peek(0) {
      Some(TT::Integer(num)) => *num,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    step
  } else {
    1
  };

  let mut variables = ph.variables.clone();
  variables.insert(variable.clone());

  let block = Box::new(block::parse(ph, variables)?);

  let node = Node::For(For::new(start, end, step, variable, block));

  Ok(node)
}

fn parse_foreach(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  let iterable = value::parse_inner(ph)?;

  let mut variables = ph.variables.clone();
  variables.insert(variable.clone());

  let block = block::parse_inner(ph, variables)?;

  let node = Node::Foreach(Foreach::new(iterable, variable, block));

  Ok(node)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::For);

  ph.advance();

  let variable = match ph.peek(0) {
    Some(TT::Identifier(variable)) => variable.clone(),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  check_token!(ph, TT::In);

  ph.advance();

  let node = match ph.peek(0) {
    Some(TT::Integer(_)) => parse_for(ph, variable)?,
    Some(TT::At | TT::LBracket | TT::Identifier(..)) => parse_foreach(ph, variable)?,
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  Ok(node)
}
