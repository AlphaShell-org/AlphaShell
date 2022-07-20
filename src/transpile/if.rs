use std::fmt::Write;

use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
  value,
};
use crate::parse::{
  node::Node,
  r#if::{Else, If},
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
