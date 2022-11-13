use super::{
  block::Block,
  error::{Error, ParserResult},
  function_call::FunctionCall,
  node::Node,
  parse_helper::ParseHelper,
  value::Value,
};
use crate::{
  check_token,
  parse::{block, value},
  types::TT,
};

#[derive(Debug, PartialEq, Clone)]
pub struct While {
  pub condition: Value,
  pub block: Block,
}

impl While {
  pub fn new(condition: Value, block: Block) -> Self {
    Self { condition, block }
  }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Clone)]
pub struct WhileLet {
  pub name: String,
  pub call: FunctionCall,
  pub block: Block,
}

impl WhileLet {
  pub fn new(name: String, call: FunctionCall, block: Block) -> Self {
    Self { name, call, block }
  }
}

fn parse_inner(ph: &mut ParseHelper) -> ParserResult<While> {
  let condition = value::parse_inner(ph)?;

  let block = block::parse_inner(ph, ph.variables.clone())?;

  let r#while = While::new(condition, block);

  Ok(r#while)
}

fn parse_with_let(ph: &mut ParseHelper) -> ParserResult<WhileLet> {
  let (name, value) = value::parse_inline_let(ph)?;

  let mut variables = ph.variables.clone();
  variables.insert(name.clone());

  let block = block::parse_inner(ph, variables)?;

  let r#while = WhileLet::new(name, value, block);

  Ok(r#while)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::While);

  ph.advance();

  match ph.peek(0) {
    Some(TT::Let) => Ok(Node::WhileLet(parse_with_let(ph)?)),
    Some(_) => Ok(Node::While(parse_inner(ph)?)),
    None => Err(Error::end(ph)),
  }
}
