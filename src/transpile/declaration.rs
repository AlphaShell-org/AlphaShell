use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::{
  node::Node,
  value::{Value, Value},
  var::{Declaration, DeclarationType},
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
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

      if let Node::Value(Value::Raw(Value::Map(_))) = **value {
        let value = value::transpile(t, value)?;
        return Ok(format!("typeset -A {name}={value}"));
      }

      let value = value::transpile(t, value)?;

      Ok(t.use_indent(&format!(r#"{type_string} {name}={value}"#)))
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
