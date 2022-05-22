use super::{
  function::Function,
  function_call::FunctionCall,
  r#for::{For, Foreach},
  r#if::If,
  r#while::While,
  value::Value,
  var::Declaration,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Value(Value),

  Source(Vec<String>),
  ImportedCode(Vec<Node>),
  Block(Vec<Node>),
  Function(Function),
  FunctionCall(FunctionCall),
  Declaration(Declaration),
  If(If),
  For(For),
  Foreach(Foreach),
  While(While),
  Return(u8),
  Continue,
  Break,
}

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}
