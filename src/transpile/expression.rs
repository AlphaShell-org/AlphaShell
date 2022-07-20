use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::node::Node;

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Expression(value) => value::transpile_inner(t, value, node),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
