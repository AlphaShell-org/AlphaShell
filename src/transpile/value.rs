use super::{
  error::{Error, TranspileResult},
  function_call,
};
use crate::parse::{
  node::Node,
  value::{BinaryOperator, Data, Expression, Value},
};

fn transpile_raw(value: &Data) -> TranspileResult<String> {
  match value {
    Data::Identifier(name) => Ok(String::new() + "${" + name + "}"),
    Data::String(string) => Ok(format!("\"{string}\"")),
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
    Data::FunctionCall(call) => {
      let call = function_call::transpile_inner(call)?;
      Ok(format!("$({call})"))
    }
  }
}

fn transpile_operator(operator: &BinaryOperator) -> String {
  let string = match operator {
    BinaryOperator::Add => "+",
    BinaryOperator::Sub => "-",
    op => todo!("{op:?}"),
  };
  string.to_owned()
}

fn transpile_expression(expression: &Expression) -> TranspileResult<String> {
  let Expression {
    left,
    operator,
    right,
  } = expression;

  Ok(transpile_value(left)? + &transpile_operator(operator) + &transpile_value(right)?)
}

fn transpile_value(value: &Value) -> TranspileResult<String> {
  match value {
    Value::Raw(value) => transpile_raw(value),
    Value::Expression(expression) => transpile_expression(expression),
  }
}

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::Value(value) => transpile_value(value),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
