use super::{
  block::{self, Block},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value::Value,
};
use crate::{check_token, parse::value, types::TT};

#[derive(Debug, PartialEq, Clone)]
pub enum Else {
  Else(Block),
  Elif(Box<If>),
}

#[derive(Debug, PartialEq, Clone)]
struct Elif(Node, Node);

#[derive(Debug, PartialEq, Clone)]
pub struct If {
  condition: Value,
  block: Block,
  r#else: Option<Else>,
}

impl If {
  pub fn new(condition: Value, block: Block, r#else: Option<Else>) -> Self {
    Self {
      condition,
      block,
      r#else,
    }
  }
}

fn parse_inner(ph: &mut ParseHelper) -> ParserResult<If> {
  let condition = value::parse_inner(ph)?;

  let block = block::parse_inner(ph)?;

  let r#else = match ph.peek(0) {
    Some(TT::Elif) => {
      ph.advance();
      Some(Else::Elif(Box::new(parse_inner(ph)?)))
    }

    Some(TT::Else) => {
      ph.advance();
      Some(Else::Else(block::parse_inner(ph)?))
    }

    _ => None,
  };

  let r#if = If::new(condition, block, r#else);

  Ok(r#if)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::If);

  ph.advance();

  let r#if = Node::If(parse_inner(ph)?);

  Ok(r#if)
}
