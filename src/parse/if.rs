use super::{
  block::{self, Block},
  error::{Error, ParserResult},
  function_call::FunctionCall,
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
  pub condition: Value,
  pub block: Block,
  pub r#else: Option<Else>,
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

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Clone)]
pub struct IfLet {
  pub name: String,
  pub call: FunctionCall,
  pub block: Block,
  pub r#else: Option<Block>,
}

impl IfLet {
  pub fn new(name: String, call: FunctionCall, block: Block, r#else: Option<Block>) -> Self {
    Self {
      name,
      call,
      block,
      r#else,
    }
  }
}

fn parse_inner(ph: &mut ParseHelper) -> ParserResult<If> {
  let condition = value::parse_inner(ph)?;

  let block = block::parse_inner(ph, ph.variables.clone())?;

  let r#else = match ph.peek(0) {
    Some(TT::Elif) => {
      ph.advance();
      Some(Else::Elif(Box::new(parse_inner(ph)?)))
    }

    Some(TT::Else) => {
      ph.advance();
      Some(Else::Else(block::parse_inner(ph, ph.variables.clone())?))
    }

    _ => None,
  };

  let r#if = If::new(condition, block, r#else);

  Ok(r#if)
}

fn parse_with_let(ph: &mut ParseHelper) -> ParserResult<IfLet> {
  let (name, value) = value::parse_inline_let(ph)?;

  let mut variables = ph.variables.clone();
  variables.insert(name.clone());

  let block = block::parse_inner(ph, variables)?;

  let r#else = match ph.peek(0) {
    Some(TT::Elif) => {
      return Err(Error::new("if let can't have elif block", ph.get(0)));
    }

    Some(TT::Else) => {
      ph.advance();
      Some(block::parse_inner(ph, ph.variables.clone())?)
    }

    _ => None,
  };

  Ok(r#IfLet::new(name, value, block, r#else))
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::If);

  ph.advance();

  match ph.peek(0) {
    Some(TT::Let) => Ok(Node::IfLet(parse_with_let(ph)?)),
    Some(_) => Ok(Node::If(parse_inner(ph)?)),
    None => Err(Error::end(ph)),
  }
}
