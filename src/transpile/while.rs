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
  r#while::{While, WhileLet},
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::While(While { condition, block }) => {
      t.push_block(BlockType::Condition);
      let condition = value::transpile_inner(t, condition, node)?;
      t.pop_block();
      let head = t.use_indent(&format!("while [[ {condition} ]]; do"));
      let block = block::transpile_inner(t, block)?;
      let end = t.use_indent("done");

      let output = format!("{head}\n{block}\n{end}");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}

pub fn transpile_let(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::WhileLet(WhileLet {
      name,
      call: value,
      block,
    }) => {
      t.push_block(BlockType::FunctionCall);
      let call = function_call::transpile_inner(t, value, node)?;
      t.pop_block();

      let temp_name = format!("__tmp_{}", random_string(6));

      t.push_block(BlockType::Generic);

      let block = block::transpile_inner(t, block)?;

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
