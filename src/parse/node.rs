use super::{
  declaration::Declaration,
  function::Function,
  function_call::FunctionCall,
  r#for::{For, Foreach},
  r#if::{If, IfLet},
  r#while::{While, WhileLet},
  value::Value,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Value(Value),

  Source(String),
  Block(Vec<Node>),
  Function(Function),
  FunctionCall(FunctionCall),
  Declaration(Declaration),
  Expression(Value),
  If(If),
  IfLet(IfLet),
  For(For),
  Foreach(Foreach),
  While(While),
  WhileLet(WhileLet),
  Return(Value),
  Continue,
  Break,

  Empty,
}

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}
