use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::{
  node::Node,
  switch::{Case, Switch},
};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::Switch(Switch { arg, cases }) = node {
    let cases = cases
      .iter()
      .map(|Case { value, block }| {
        let head = t.use_indent(value);
        let block = block::transpile(t, block)?;
        let end = t.use_indent(";;");

        Ok(format!("{head})\n{block}\n{end}"))
      })
      .collect::<TranspileResult<Vec<String>>>()?
      .join("\n");

    let arg = value::transpile_inner(t, arg, node)?;

    Ok(format!("case {arg} in\n{cases}\nesac",))
  } else {
    Err(Error::invalid(node))
  }
}
