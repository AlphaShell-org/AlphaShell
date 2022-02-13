use super::{
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
};
use crate::parse::node::Node;

pub fn transpile_inner(t: &mut Transpiler, block: &[Node]) -> TranspileResult<String> {
  if t.get_block() != &Some(BlockType::Import) {
    t.indent();
  }

  let block = super::inner(block, t)?;

  if t.get_block() != &Some(BlockType::Import) {
    t.deindent();
  }

  Ok(block)
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Block(block) => transpile_inner(t, block),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
