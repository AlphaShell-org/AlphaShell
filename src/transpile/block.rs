use super::{
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
};
use crate::parse::node::Node;

pub fn transpile_inner(t: &mut Transpiler, block: &[Node]) -> TranspileResult<String> {
  t.push_block(BlockType::Generic);
  let block = super::inner(block, t)?;
  t.pop_block();

  Ok(block)
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Block(block) = node {
    transpile_inner(t, block)
  } else {
    Err(Error::invalid(node))
  }
}
