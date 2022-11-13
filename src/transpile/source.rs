use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::node::Node;

pub fn transpile(_t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Source(file) = node {
    Ok(format!("source '{file}'"))
  } else {
    Err(Error::invalid(node))
  }
}
