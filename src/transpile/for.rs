use super::{
  block,
  error::{Error, TranspileResult},
};
use crate::parse::{node::Node, r#for::For};

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::For(For {
      start,
      end,
      step,
      variable,
      block,
    }) => {
      let block = block::transpile(block)?;

      let output = format!("for {variable} in {{{start}..{end}..{step}}}; do\n{block}\ndone");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
