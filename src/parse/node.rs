use crate::types::TokenType;

use super::{
  expression::Expression, function::Function, function_call::FunctionCall, import::Import,
  var::Declaration,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Identifier(String),
  Operator(TokenType),
  String(String),
  Int(i64),
  Float(f64),
  Expression(Expression),
  Array(Vec<String>),
  Map,
  Import(Import),
  Block(Vec<Node>),
  Function(Function),
  FunctionCall(FunctionCall),
  Declaration(Declaration),
  IndexCall,
  If,
  For,
  While,
  Return(Option<u8>),
  Continue,
  Break,
}
