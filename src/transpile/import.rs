use super::error::{Error, TranspileResult};

use crate::parse::node::Node;

pub fn transpile(node: &Node) -> TranspileResult<String> {
  // hello, test comment
  match node {
    Node::Import(files) => {
      let output = files
        .iter()
        .map(|file| format!("source '{file}'"))
        .collect::<Vec<_>>()
        .join(";\n");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
