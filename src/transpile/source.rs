use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::node::Node;

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Source(files) => {
      let output = files
        .iter()
        .map(|file| t.use_indent(&format!("source '{file}'")))
        .collect::<Vec<_>>()
        .join(";\n");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
