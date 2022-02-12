use super::{
  block,
  error::{Error, TranspileResult},
};
use crate::parse::{function::Function, node::Node};

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Function(Function {
      name,
      params,
      block,
    }) => {
      let block = block::transpile(block)?;
      let params = params
        .iter()
        .enumerate()
        .map(|(i, param)| format!("local {param}=${i}\n"))
        .collect::<String>();

      let output = format!("function {name}() {{\n{params}\n{block}\n}}");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
