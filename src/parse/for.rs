use crate::{check_token, types::TT};

use super::{
  block,
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value,
};

#[derive(Debug, PartialEq, Clone)]
pub struct For {
  start: i32,
  end: i32,
  step: i32,
  variable: String,
  block: Box<Node>,
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
  variable: String,
  iterable: Box<Node>,
  block: Box<Node>,
}

impl Foreach {
  pub fn new(iterable: Box<Node>, variable: String, block: Box<Node>) -> Self {
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

  let block = Box::new(block::parse(ph)?);

  let node = Node::For(For::new(start, end, step, variable, block));

  Ok(node)
}

fn parse_foreach(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  let iterable = Box::new(value::parse(ph)?);

  let block = Box::new(block::parse(ph)?);

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
