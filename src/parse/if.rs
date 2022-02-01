use crate::{
  check_token,
  parse::{block, error::Error, expression},
  types::TokenType,
};

use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

#[derive(Debug, PartialEq, Clone)]
struct Elif(Node, Node);

#[derive(Debug, PartialEq, Clone)]
pub struct If {
  condition: Box<Node>,
  block: Box<Node>,
  r#else: Option<Box<Node>>,
}

impl If {
  pub fn new(condition: Box<Node>, block: Box<Node>, r#else: Option<Box<Node>>) -> Self {
    Self {
      condition,
      block,
      r#else,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::If);
  ph.advance();

  let condition = expression::parse(ph)?;

  check_token!(ph, TokenType::LBrace);
  ph.advance();

  let block = block::parse(ph)?;

  check_token!(ph, TokenType::RBrace);
  ph.advance();

  let r#else = match ph.peek(0) {
    Some(TokenType::Elif) => Some(Box::new(parse(ph)?)),
    Some(TokenType::Else) => Some(Box::new(block::parse(ph)?)),
    _ => None,
  };

  let r#if = Node::If(If::new(Box::new(condition), Box::new(block), r#else));

  Ok(r#if)
}
