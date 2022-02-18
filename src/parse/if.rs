use super::{
  block::{self, Block},
  condition::{self, Condition},
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};
use crate::{check_token, types::TT};

#[derive(Debug, PartialEq, Clone)]
enum Else {
  Else(Block),
  Elif(If),
}

#[derive(Debug, PartialEq, Clone)]
struct Elif(Node, Node);

#[derive(Debug, PartialEq, Clone)]
pub struct If {
  condition: Condition,
  block: Block,
  r#else: Option<Box<Else>>,
}

impl If {
  pub fn new(condition: Condition, block: Block, r#else: Option<Box<Else>>) -> Self {
    Self {
      condition,
      block,
      r#else,
    }
  }
}

fn parse_inner(ph: &mut ParseHelper) -> ParserResult<If> {
  check_token!(ph, TT::If);

  ph.advance();

  let condition = condition::parse(ph)?;

  check_token!(ph, TT::LBrace);

  ph.advance();

  let block = block::parse_inner(ph)?;

  check_token!(ph, TT::RBrace);

  ph.advance();

  let r#else = match ph.peek(0) {
    Some(TT::Elif) => Some(Else::Elif(parse_inner(ph)?)),
    Some(TT::Else) => Some(Else::Else(block::parse_inner(ph)?)),
    _ => None,
  };

  let r#else = r#else.map(|val| Box::new(val));

  let r#if = If::new(condition, block, r#else);

  Ok(r#if)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  let r#if = Node::If(parse_inner(ph)?);

  Ok(r#if)
}
