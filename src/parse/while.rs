use super::{
  block::Block,
  error::{Error, ParserResult},
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

fn parse_inner(ph: &mut ParseHelper) -> ParserResult<While> {
  check_token!(ph, TT::While);

  ph.advance();

  let condition = value::parse_inner(ph)?;

  let block = block::parse_inner(ph)?;

  let r#while = While::new(condition, block);

  Ok(r#while)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  let r#while = Node::While(parse_inner(ph)?);

  Ok(r#while)
}
