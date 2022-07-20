use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
  value,
};
use crate::parse::{node::Node, r#while::While};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::While(While { condition, block }) => {
      t.push_block(BlockType::Condition);
      let condition = value::transpile_inner(t, condition, node)?;
      t.pop_block();
      let head = t.use_indent(&format!("while [[ {condition} ]]; do"));
      let block = block::transpile_inner(t, block)?;
      let end = t.use_indent("done");

      let output = format!("{head}\n{block}\n{end}");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
