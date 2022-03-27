use super::{
  error::{Error, TranspileResult},
  function_call,
  transpiler::{BlockType, Transpiler},
};
use crate::parse::{
  node::Node,
  value::{BinaryOperator, Value, Expression, Value},
};

fn transpile_raw(t: &mut Transpiler, value: &Value) -> TranspileResult<String> {
  match value {
    Value::Identifier(name) => {
      if t.get_block() == &Some(BlockType::Foreach) && name != "@" {
        Ok(format!("${{(@){name}}}"))
      } else {
        Ok(format!("${{{name}}}"))
      }
    }
    Value::String(string) => Ok(format!("\"{string}\"")),
    Value::Int(num) => Ok(num.to_string()),
    Value::Float(num) => Ok(num.to_string()),
    Value::Array(array) => {
      if t.get_block() == &Some(BlockType::Foreach) {
        Ok(
          array
            .iter()
            .map(|item| format!("'{item}'"))
            .collect::<Vec<_>>()
            .join(" "),
        )
      } else {
        Ok(format!(
          "({})",
          array
            .iter()
            .map(|item| format!("'{item}'"))
            .collect::<Vec<_>>()
            .join(" ")
        ))
      }
    }
    Value::Map(map) => {
      // typeset -A assoc=([key1]=value1 [key2]=value2)
      Ok(format!(
        "({})",
        map
          .iter()
          .map(|(k, v)| format!("[{k}]={v}"))
          .collect::<Vec<_>>()
          .join(" ")
      ))
    }
    Value::FunctionCall(call) => {
      let call = function_call::transpile_inner(t, call)?;
      Ok(format!("$({call})"))
    }
  }
}

fn transpile_operator(_t: &mut Transpiler, operator: &BinaryOperator) -> String {
  let string = match operator {
    BinaryOperator::Add => "+",
    BinaryOperator::Sub => "-",
    op => todo!("{op:?}"),
  };
  string.to_owned()
}

fn transpile_expression(t: &mut Transpiler, expression: &Expression) -> TranspileResult<String> {
  let Expression {
    left,
    operator,
    right,
  } = expression;

  Ok(transpile_value(t, left)? + &transpile_operator(t, operator) + &transpile_value(t, right)?)
}

fn transpile_value(t: &mut Transpiler, value: &Value) -> TranspileResult<String> {
  match value {
    Value::Raw(value) => transpile_raw(t, value),
    Value::Expression(expression) => transpile_expression(t, expression),
  }
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Value(value) => transpile_value(t, value),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
