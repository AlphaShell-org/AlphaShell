use super::error::{Error, TranspileResult};
use crate::parse::{
  node::Node,
  var::{Declaration, DeclarationType},
};

pub fn transpile_block(block: &Vec<Node>) -> TranspileResult<String> {
  let block = super::transpile(block)?;

  Ok(block)
}

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Block(block) => transpile_block(block),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
