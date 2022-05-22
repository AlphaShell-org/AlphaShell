use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::{node::Node, r#while::While};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::While(While { condition, block }) => {
      todo!()
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
