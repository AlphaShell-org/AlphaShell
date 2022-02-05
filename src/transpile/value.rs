use crate::parse::{
  function_call::FunctionCall,
  node::Node,
  value::{Data, Expression},
  var::Declaration,
};
use crate::parse::{value::Value, var::DeclarationType};

use super::error::{Error, TranspileResult};

fn transpile_raw(value: &Data) -> TranspileResult<String> {
  match value {
    Data::Identifier(name) => Ok("".to_owned() + "${" + name + "}"),
    Data::String(string) => Ok(string.clone()),
    Data::Int(num) => Ok(num.to_string()),
    Data::Float(num) => Ok(num.to_string()),
    Data::Array(array) => Ok(format!(
      "({})",
      array
        .iter()
        .map(|item| format!("'{item}'"))
        .collect::<Vec<_>>()
        .join(" ")
    )),
    Data::Map(_) => todo!(),
    Data::FunctionCall(FunctionCall {
      name,
      args,
      is_daemon,
      next,
    }) => {
      todo!()
    }
  }
}

fn transpile_expression(expression: &Expression) -> TranspileResult<String> {
  todo!("{expression:?}")
}

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Value(value) => match value {
      Value::Raw(value) => transpile_raw(value),
      Value::Expression(expression) => transpile_expression(expression),
    },
    _ => Err(Error::new("Invalid node type", node)),
  }
}
