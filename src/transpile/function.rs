use super::{
  block,
  error::{Error, TranspileResult},
  transpiler::Transpiler,
};
use crate::parse::{function::Function, node::Node};

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Function(Function {
      name,
      params,
      block,
    }) => {
      let head = t.use_indent(&format!("function {name}() {{"));

      t.indent();
      let params = params
        .iter()
        .enumerate()
        .map(|(i, param)| t.use_indent(&format!("local {param}=${i}\n")))
        .collect::<String>();
      t.deindent();

      let block = block::transpile(t, block)?;
      let end = t.use_indent("}");

      let output = format!("{head}\n{params}{block}\n{end}");

      Ok(output)
    }
    _ => Err(Error::new("Invalid node type", node)),
  }
}
