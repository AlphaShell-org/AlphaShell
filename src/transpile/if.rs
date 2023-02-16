use std::fmt::Write;

use super::{
  block,
  error::{Error, TranspileResult},
  function_call,
  transpiler::{BlockType, Transpiler},
  utils::random_string,
  value,
};
use crate::parse::{
  node::Node,
  r#if::{Else, If, IfLet},
  value::Value,
};

pub fn transpile_condition(
  t: &mut Transpiler,
  condition: &Value,
  node: &Node,
) -> TranspileResult<String> {
  t.push_block(BlockType::Condition);
  let string = value::transpile_inner(t, condition, node)?;
  t.pop_block();

  if matches!(condition, Value::FunctionCall(..)) {
    Ok(string)
  } else {
    Ok(format!("[[ {string} ]]"))
  }
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::If(If {
    condition,
    r#else,
    block,
  }) = node
  {
    let condition = transpile_condition(t, condition, node)?;
    let head = t.use_indent(&format!("if {condition}; then"));
    let block = block::transpile_inner(t, block)?;

    let mut string_else = String::new();

    let mut outer_else = r#else;
    while let Some(r#else) = outer_else {
      match r#else {
        Else::Else(block) => {
          writeln!(string_else, "{}", t.use_indent("else")).unwrap();
          let block = block::transpile_inner(t, r#block)?;
          writeln!(string_else, "{block}").unwrap();
          r#outer_else = &None;
        }
        Else::Elif(r#if) => {
          r#outer_else = &r#if.r#else;

          let condition = transpile_condition(t, &r#if.condition, node)?;

          writeln!(string_else, "{}", &format!("elif {condition}; then")).unwrap();

          let block = block::transpile_inner(t, &r#if.r#block)?;

          writeln!(string_else, "{block}").unwrap();
        }
      }
    }

    let end = t.use_indent("fi");

    let mut output = String::new();
    writeln!(output, "{head}").unwrap();
    writeln!(output, "{block}").unwrap();
    if !string_else.is_empty() {
      write!(output, "{string_else}").unwrap();
    }
    write!(output, "{end}").unwrap();

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}

pub fn transpile_let(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::IfLet(IfLet {
    name,
    call,
    block,
    r#else,
  }) = node
  {
    t.push_block(BlockType::FunctionCall);
    let call = function_call::transpile_inner(t, call, node)?;
    t.pop_block();

    let temp_name = format!("__tmp_{}", random_string(6));

    t.push_block(BlockType::Generic);
    let block = block::transpile_inner(t, block)?;
    t.pop_block();

    let output = if let Some(else_block) = r#else {
      let else_block = block::transpile_inner(t, else_block)?;
      format!(
        "
{temp_name}(){{
  local {name}
  {name}=$({call})
  if [ $? = 0 ]; then
{block}
  else
{else_block}
  fi
}}
{temp_name}"
      )
    } else {
      format!(
        "
{temp_name}(){{
  local {name}
  {name}=$({call})
  if [ $? = 0 ]; then
{block}
  fi
}}
{temp_name}"
      )
    };

    let output = output
      .trim()
      .lines()
      .map(|line| t.use_indent(line))
      .collect::<Vec<_>>()
      .join("\n");

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}
