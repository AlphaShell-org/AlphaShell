use super::{
  block,
  error::{Error, TranspileResult},
  function_call,
  r#if::transpile_condition,
  transpiler::{BlockType, Transpiler},
  utils::random_string,
};
use crate::parse::{
  node::Node,
  r#while::{While, WhileLet},
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::While(While { condition, block }) = node {
    let condition = transpile_condition(t, condition, node)?;
    let head = t.use_indent(&format!("while {condition}; do"));
    let block = block::transpile_inner(t, block)?;
    let end = t.use_indent("done");

    let output = format!("{head}\n{block}\n{end}");

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}

pub fn transpile_let(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::WhileLet(WhileLet {
    name,
    call: value,
    block,
  }) = node
  {
    t.push_block(BlockType::FunctionCall);
    let call = function_call::transpile_inner(t, value, node)?;
    t.pop_block();

    let temp_name = format!("__tmp_{}", random_string(6));

    t.push_block(BlockType::Generic); // additional indent
    let block = block::transpile_inner(t, block)?;
    t.pop_block();

    let output = format!(
      "
{temp_name}(){{
  local {name}
  while true; do
    {name}=$({call})
    if [ ! $? = 0 ]; then
      break
    fi
{block}
  done
}}
{temp_name}"
    );

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
