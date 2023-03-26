use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::{
  declaration::{Declaration, Type},
  node::Node,
  value::{Literal, Value},
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Declaration(Declaration {
    r#type,
    name,
    value,
  }) = node
  {
    let type_string = match r#type {
      Type::Export => "export",
      Type::Let => "local",
    };

    if let Node::Value(Value::Literal(Literal::Map(_))) = **value {
      let value = value::transpile(t, value)?;
      return Ok(format!("{type_string} {name}\ntypeset -A {name}={value}"));
    }

    let value = value::transpile(t, value)?;

    Ok(t.use_indent(&format!(r#"{type_string} {name}={value}"#)))
  } else {
    Err(Error::invalid(node))
  }
}
