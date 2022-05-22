use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::{node::Node, r#if::If};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::If(If {
      condition,
      r#else,
      block,
    }) => {
      todo!()
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
