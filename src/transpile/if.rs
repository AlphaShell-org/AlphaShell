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
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::If(If {
      condition,
      r#else,
      block,
    }) => {
      t.push_block(BlockType::Condition);
      let condition = value::transpile_inner(t, condition, node)?;
      t.pop_block();
      let head = t.use_indent(&format!("if [[ {condition} ]]; then"));
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

            t.push_block(BlockType::Condition);
            let condition = value::transpile_inner(t, &r#if.condition, node)?;
            t.pop_block();

            writeln!(string_else, "{}", &format!("elif [[ {condition} ]]; then")).unwrap();

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
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}

pub fn transpile_let(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::IfLet(IfLet {
      name,
      call,
      block,
      r#else,
    }) => {
      t.push_block(BlockType::FunctionCall);
      let call = function_call::transpile_inner(t, call, node)?;
      t.pop_block();

      let temp_name = format!("__tmp_{}", random_string(6));

      t.push_block(BlockType::Generic);

      let output = if let Some(else_block) = r#else {
        format!(
          "
{temp_name}(){{
  local {name}
  {name}=$({call})
  if [ $? = 0 ]; then
{}
  else
{}
  fi
}}
{temp_name}",
          block::transpile_inner(t, block)?,
          block::transpile_inner(t, else_block)?
        )
      } else {
        format!(
          "
{temp_name}(){{
  local {name}
  {name}=$({call})
  if [ $? = 0 ]; then
{}
  fi
}}
{temp_name}",
          block::transpile_inner(t, block)?
        )
      };

      t.pop_block();

      let output = output
        .trim()
        .lines()
        .map(|line| t.use_indent(line))
        .collect::<Vec<_>>()
        .join("\n");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
