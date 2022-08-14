use std::fmt::Write;

use super::{
  error::{Error, TranspileResult},
  function_call,
  transpiler::{BlockType, Transpiler},
};
use crate::parse::{
  node::Node,
  value::{AssignmentOperator, BinaryOperator, Literal, UnaryOperator, Value},
};

fn transpile_identifier(t: &mut Transpiler, name: &String) -> String {
  if t.search(&BlockType::Identifier) || t.search(&BlockType::Arithmetics) {
    name.clone()
  } else {
    format!("\"${{{name}}}\"")
  }
}

fn transpile_literal(t: &mut Transpiler, value: &Literal, node: &Node) -> TranspileResult<String> {
  match value {
    Literal::String(string) => {
      if t.search(&BlockType::Arithmetics) {
        eprintln!("Warning: String literal inside arithmetic context");
      }

      if t.search(&BlockType::Raw) {
        Ok(string.clone())
      } else {
        Ok(format!("\"{string}\""))
      }
    }
    Literal::RawString(string) => {
      if t.search(&BlockType::Arithmetics) {
        eprintln!("Warning: RawString literal inside arithmetic context");
      }

      if t.search(&BlockType::Raw) {
        Ok(string.clone())
      } else {
        Ok(format!("'{string}'"))
      }
    }
    Literal::Int(num) => {
      if t.search(&BlockType::Arithmetics) || t.search(&BlockType::Raw) {
        Ok(num.to_string())
      } else {
        eprintln!("Warning: Integer literal outside arithmetic context");
        Ok(format!("\"{num}\""))
      }
    }
    Literal::Float(num) => Ok(num.to_string()),
    Literal::Bool(boolean) => Ok(if *boolean {
      "/bin/true".to_string()
    } else {
      "/bin/false".to_string()
    }),
    Literal::Array(array) => {
      let mut items = Vec::new();

      for item in array {
        items.push(transpile_inner(t, item, node)?);
      }

      let items = items.join(" ");

      if matches!(t.get_block(), Some(BlockType::Foreach)) {
        Ok(items)
      } else {
        Ok(format!("({items})"))
      }
    }
    Literal::Map(map) => {
      // typeset -A assoc=([key1]=value1 [key2]=value2)

      let mut transpiled = String::new();

      for (key, value) in map {
        write!(transpiled, "[{key}]={} ", transpile_inner(t, value, node)?).unwrap();
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
  if matches!(t.get_block(), Some(BlockType::Arithmetics)) {
    let operator = match operator {
      BinaryOperator::Add => "+",
      BinaryOperator::Sub => "-",
      BinaryOperator::Multiply => "*",
      BinaryOperator::Divide => "/",
      BinaryOperator::Modulo => "%",
      op => unimplemented!("{op:?}"),
    };

    Ok(format!(
      "{} {operator} {}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  } else if t.search(&BlockType::Condition) {
    let operator = match operator {
      BinaryOperator::Equal => "==",
      BinaryOperator::NotEqual => "!=",
      BinaryOperator::Greater => "-gt",
      BinaryOperator::GreaterEqual => "-ge",
      BinaryOperator::Less => "-lt",
      BinaryOperator::LessEqual => "-le",
      BinaryOperator::RegexMatch => "=~",
      BinaryOperator::And => "&&",
      BinaryOperator::Or => "||",
      op => unimplemented!("{op:?}"),
    };

    Ok(format!(
      "{} {operator} {}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  } else {
    // string mode
    match operator {
      BinaryOperator::Add => "",
      _ => {
        return Err(Error::new(
          &format!("Operator not supported in string mode: {operator:?}"),
          node,
        ))
      }
    };

    Ok(format!(
      "{}{}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  }
}

fn transpile_ternary_expression(
  t: &mut Transpiler,
  condition: &Value,
  left: &Value,
  right: &Value,
  node: &Node,
) -> TranspileResult<String> {
  t.push_block(BlockType::Condition);
  let condition = transpile_inner(t, condition, node)?;
  t.pop_block();

  let left = transpile_inner(t, left, node)?;
  let right = transpile_inner(t, right, node)?;

  let transpiled = format!("if [[ {condition} ]]; then; echo {left}; else; echo {right}; fi");

  if matches!(t.get_block(), Some(BlockType::Generic)) {
    Ok(transpiled)
  } else {
    Ok(format!("$({transpiled})"))
  }
}

fn transpile_member_expression(
  t: &mut Transpiler,

  left: &Value,
  right: &Value,
  node: &Node,
) -> TranspileResult<String> {
  t.push_block(BlockType::Identifier);
  let left = transpile_inner(t, left, node)?;
  t.pop_block();

  t.push_block(BlockType::Raw);
  let right = transpile_inner(t, right, node)?;
  t.pop_block();

  let transpiled = format!("{left}[{right}]");

  if matches!(
    t.get_block(),
    Some(BlockType::Generic | BlockType::Identifier)
  ) {
    Ok(transpiled)
  } else {
    Ok(format!("${{{transpiled}}}"))
  }
}

fn transpile_assignment(
  t: &mut Transpiler,
  left: &Value,
  operator: &AssignmentOperator,
  right: &Value,
  node: &Node,
) -> TranspileResult<String> {
  if !matches!(left, Value::Identifier(..) | Value::MemberExpression(..)) {
    return Err(Error::new("Cannot assign to this expression", node));
  }

  if matches!(t.get_block(), Some(BlockType::Arithmetics)) {
    let operator = match operator {
      AssignmentOperator::Assignment => "=",
      AssignmentOperator::AddAssignment => "+=",
      AssignmentOperator::SubAssignment => "-=",
      AssignmentOperator::MultiplyAssignment => "*=",
      AssignmentOperator::DivideAssignment => "/=",
      AssignmentOperator::ModuloAssignment => "%=",
      AssignmentOperator::PowerAssignment => "**=",
    };

    Ok(format!(
      "{} {operator} {}",
      transpile_inner(t, left, node)?,
      transpile_inner(t, right, node)?
    ))
  } else {
    // string mode
    let operator = match operator {
      AssignmentOperator::AddAssignment => "+=",
      AssignmentOperator::Assignment => "=",
      _ => {
        return Err(Error::new(
          &format!("Operator not supported in string mode: {operator:?}"),
          node,
        ))
      }
    };

    t.push_block(BlockType::Identifier);
    let left = transpile_inner(t, left, node)?;
    t.pop_block();

    let right = transpile_inner(t, right, node)?;

    Ok(format!("{left} {operator} {right}",))
  }
}

pub fn transpile_inner(t: &mut Transpiler, value: &Value, node: &Node) -> TranspileResult<String> {
  let indent = !matches!(
    t.get_block(),
    Some(
      BlockType::Expression
        | BlockType::Arithmetics
        | BlockType::Identifier
        | BlockType::Foreach
        | BlockType::Condition
    )
  );

  if indent {
    t.push_block(BlockType::Expression);
  }

  let value = match value {
    Value::Literal(value) => transpile_literal(t, value, node),
    Value::Identifier(name) => Ok(transpile_identifier(t, name)),
    Value::UnaryExpression(operator, right) => transpile_unary_expression(t, operator, right, node),
    Value::BinaryExpression(left, operator, right) => {
      transpile_binary_expression(t, left, operator, right, node)
    }
    Value::TernaryExpression(condition, left, right) => {
      transpile_ternary_expression(t, condition, left, right, node)
    }
    Value::MemberExpression(left, right) => transpile_member_expression(t, left, right, node),
    Value::Assignment(left, operator, right) => {
      transpile_assignment(t, left, operator, right, node)
    }
    Value::FunctionCall(function_call) => function_call::transpile_inner(t, function_call, node),
  };

  if indent {
    t.pop_block();
  }

  value
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Value(value) => transpile_inner(t, value, node),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
