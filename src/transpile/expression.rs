use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::node::Node;

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Expression(value) = node {
    value::transpile_inner(t, value, node)
  } else {
    Err(Error::invalid(node))
  }
}
