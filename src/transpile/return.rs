use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::node::Node;

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Return(value) = node {
    let output = format!("return {}", value::transpile_inner(t, value, node)?);

    Ok(t.use_indent(&output))
  } else {
    Err(Error::invalid(node))
  }
}
