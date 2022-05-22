use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::node::Node;

pub fn transpile(_t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Source(file) => Ok(format!("source '{file}'")),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
