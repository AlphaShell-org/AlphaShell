use super::{
  block,
  error::{Error, TranspileResult},
  value,
};
use crate::parse::{node::Node, r#for::Foreach};

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Foreach(Foreach {
      variable,
      iterable,
      block,
    }) => {
      let block = block::transpile(block)?;

      let iterable = value::transpile(iterable)?;

      let output = format!("for {variable} in {iterable}; do\n{block}\ndone");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
