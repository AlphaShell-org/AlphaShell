use crate::types::TokenType;

use super::{
  expression::Expression,
  function::Function,
  function_call::FunctionCall,
  import::Import,
  r#for::{For, Foreach},
  r#if::If,
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
  Map(Vec<(String, String)>),
  Import(Import),
  Block(Vec<Node>),
  Function(Function),
  FunctionCall(FunctionCall),
  Declaration(Declaration),
  IndexCall,
  If(If),
  For(For),
  Foreach(Foreach),
  While(),
  Return(Option<u8>),
  Continue,
  Break,
}
