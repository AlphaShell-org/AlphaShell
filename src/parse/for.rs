use super::{
  block::{self, Block},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value::{self, Value},
};
use crate::{check_token, parse::value::Literal, types::TT};

#[derive(Debug, PartialEq, Clone)]
pub struct For {
  pub start: Value,
  pub end: Value,
  pub step: Value,
  pub variable: String,
  pub block: Box<Node>,
}

impl For {
  pub fn new(start: Value, end: Value, step: Value, variable: String, block: Box<Node>) -> Self {
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
  let start = value::parse_inner(ph)?;

  check_token!(ph, TT::Range);
  ph.advance();

  let end = value::parse_inner(ph)?;

  let step = if let Some(TT::Range) = ph.peek(0) {
    ph.advance();
    value::parse_inner(ph)?
  } else {
    Value::Literal(Literal::Int(1))
  };

  let mut variables = ph.variables.clone();
  variables.insert(variable.clone());

  let block = Box::new(block::parse(ph, variables)?);

  let node = Node::For(For::new(start, end, step, variable, block));

  Ok(node)
}

fn parse_foreach(ph: &mut ParseHelper, variable: String) -> ParserResult<Node> {
  let iterable = value::parse_inner(ph).context("Parsing iterable")?;

  let mut variables = ph.variables.clone();
  variables.insert(variable.clone());

  let block = block::parse_inner(ph, variables).context("Parsing foreach body")?;

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
    Some(TT::At | TT::LBracket | TT::Identifier(..) | TT::String(..)) => {
      parse_foreach(ph, variable)?
    }
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  Ok(node)
}
