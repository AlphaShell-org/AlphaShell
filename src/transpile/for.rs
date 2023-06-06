use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::{node::Node, r#for::For};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::For(For {
    start,
    end,
    step,
    variable,
    block,
  }) = node
  {
    let start = value::transpile_inner(t, start, node)?;
    let end = value::transpile_inner(t, end, node)?;
    let step = value::transpile_inner(t, step, node)?;
    let head = t.use_indent(&format!("for {variable} ({{{start}..{end}..{step}}}); do"));
    let block = block::transpile(t, block)?;
    let end = t.use_indent("done");

    let output = format!("{head}\n{block}\n{end}");

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}
