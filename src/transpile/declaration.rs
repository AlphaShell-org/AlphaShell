use super::error::{Error, TranspileResult};
use crate::parse::{
  node::Node,
  var::{Declaration, DeclarationType},
};

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Declaration(Declaration {
      r#type,
      name,
      value,
    }) => {
      let type_string = match r#type {
        DeclarationType::Export => "export",
        DeclarationType::Let => "local",
      };

      let value = super::value::transpile(value)?;

      Ok(format!(r#"{type_string} {name}={value}"#))
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
