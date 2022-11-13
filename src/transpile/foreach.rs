use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
  value,
};
use crate::parse::{node::Node, r#for::Foreach};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Foreach(Foreach {
    variable,
    iterable,
    block,
  }) = node
  {
    t.push_block(BlockType::Foreach);
    let iterable = value::transpile_inner(t, iterable, node)?;
    t.pop_block();

    let head = t.use_indent(&format!("for {variable} ({iterable}); do"));
    let block = block::transpile_inner(t, block)?;
    let end = t.use_indent("done");

    let output = format!("{head}\n{block}\n{end}");

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}
