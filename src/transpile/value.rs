use std::fmt::Write;

use super::{
  error::{Error, TranspileResult},
  function_call,
  transpiler::{BlockType, Transpiler},
};
use crate::parse::{
  node::Node,
  value::{BinaryOperator, Literal, UnaryOperator, Value},
};

fn transpile_identifier(t: &mut Transpiler, name: &String) -> String {
  if let Some(BlockType::Arithmetics) = t.get_block() {
    name.clone()
  } else {
    format!("${{{}}}", name)
  }
}

fn transpile_literal(t: &mut Transpiler, value: &Literal, node: &Node) -> TranspileResult<String> {
  match value {
    Literal::String(string) => Ok(format!("\"{string}\"")),
    Literal::Int(num) => {
      eprintln!("Warning: Integer literal outside arithemetic context");
      Ok(num.to_string())
    }
    Literal::Float(num) => Ok(num.to_string()),
    Literal::Array(array) => {
      let mut items = Vec::new();

      for item in array {
        items.push(transpile_inner(t, item, node)?);
      }

      let items = items.join(" ");

      if t.get_block() == Some(&BlockType::Foreach) {
        Ok(items)
      } else {
        Ok(format!("({items})"))
      }
    }
    Literal::Map(map) => {
      // typeset -A assoc=([key1]=value1 [key2]=value2)

      let mut transpiled = String::new();

      for (key, value) in map {
        write!(
          transpiled,
          "[{key}]='{}' ",
          transpile_inner(t, value, node)?
        )
        .unwrap();
      }

      Ok(format!("({})", transpiled))
    }
  }
}

fn transpile_unary_expression(
  t: &mut Transpiler,
  operator: &UnaryOperator,
  right: &Value,
  node: &Node,
) -> TranspileResult<String> {
  let operator = match operator {
    UnaryOperator::Not => "!",
    UnaryOperator::Minus => "-",
  };

  Ok(operator.to_owned() + &transpile_inner(t, right, node)?)
}

fn transpile_binary_expression(
  t: &mut Transpiler,
  left: &Value,
  operator: &BinaryOperator,
  right: &Value,
  node: &Node,
) -> TranspileResult<String> {
  if t.get_block() == Some(&BlockType::Arithmetics) {
    let operator = match operator {
      BinaryOperator::Add => "+",
      BinaryOperator::Sub => "-",
      op => todo!("{op:?}"),
    };

    Ok(format!(
      "{} {operator} {}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  } else {
    match operator {
      BinaryOperator::Add => {}
      _ => {
        return Err(Error::new(
          &format!("Operator not supported in string mode: {operator:?}"),
          node,
        ))
      }
    }

    Ok(format!(
      "{}{}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  }
}

pub fn transpile_inner(t: &mut Transpiler, value: &Value, node: &Node) -> TranspileResult<String> {
  let indent = !matches!(
    t.get_block(),
    Some(BlockType::Expression | BlockType::Arithmetics)
  );

  if indent {
    t.indent(BlockType::Expression);
  }

  let value = match value {
    Value::Literal(value) => transpile_literal(t, value, node),
    Value::Identifier(name) => Ok(transpile_identifier(t, name)),
    Value::UnaryExpression(operator, right) => transpile_unary_expression(t, operator, right, node),
    Value::BinaryExpression(left, operator, right) => {
      transpile_binary_expression(t, left, operator, right, node)
    }
    Value::TernaryExpression(_, _, _) => todo!(),
    Value::MemberExpression(_, _) => todo!(),
    Value::FunctionCall(function_call) => function_call::transpile_inner(t, function_call, node),
  };

  if indent {
    t.deindent();
  }

  value
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Value(value) => transpile_inner(t, value, node),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
