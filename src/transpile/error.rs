use std::fmt;

use crate::parse::node::Node;

pub type TranspileResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
  pub msg: String,
  pub node: Node,
}

impl Error {
  pub fn new(msg: &str, node: &Node) -> Self {
    Error {
      msg: msg.to_string(),
      node: node.clone(),
    }
  }

  pub fn invalid(node: &Node) -> Self {
    Self::new("Invalid node type", node)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Error { msg, node } = self;
    write!(f, "TranspileError: \"{msg}\" at node {node}")
  }
}
