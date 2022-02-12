use super::error::{Error, TranspileResult};
use crate::parse::node::Node;

pub fn transpile_inner(block: &[Node]) -> TranspileResult<String> {
  let block = super::transpile(block)?;

  Ok(block)
}

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Block(block) => transpile_inner(block),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
