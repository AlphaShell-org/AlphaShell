use crate::types::TokenType;

use super::{
  block::Block,
  expression::Expression,
  function::Function,
  import::Import,
  number::{Float, Int},
  string::Str,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Operator {
  r#type: TokenType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
  r#type: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Identifier(Identifier),
  Operator(Operator),
  String(Str),
  Int(Int),
  Float(Float),
  Expression(Expression),
  Import(Import),
  Block(Block),
  Function(Function),
  IndexCall,
  If,
  For,
  While,
  Return,
  Continue,
  Break,
  Array,
}
