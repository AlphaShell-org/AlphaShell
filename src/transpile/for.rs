use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::Transpiler,
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
    let head = t.use_indent(&format!("for {variable} in {{{start}..{end}..{step}}}; do"));
    let block = block::transpile(t, block)?;
    let end = t.use_indent("done");

    let output = format!("{head}\n{block}\n{end}");

    Ok(output)
  } else {
    Err(Error::invalid(node))
  }
}
