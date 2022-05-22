mod error;
mod transpiler;
// mod utils;

use error::TranspileResult;

use self::transpiler::Transpiler;
use crate::parse::node::Node::{
  self, Block, Break, Continue, Declaration, Empty, For, Foreach, Function, FunctionCall, If,
  Return, Source, Value, While,
};

mod block;
mod declaration;
mod r#for;
mod foreach;
mod function;
mod function_call;
mod r#if;
mod source;
mod value;
mod r#while;

pub fn transpile(tree: &[Node]) -> TranspileResult<String> {
  let mut t = Transpiler::new("  ");

  inner(tree, &mut t)
}

pub fn inner(tree: &[Node], t: &mut Transpiler) -> TranspileResult<String> {
  let mut output = Vec::new();

  for node in tree {
    let code = match node {
      Block(_) => block::transpile(t, node),
      Declaration(_) => declaration::transpile(t, node),
      For(_) => r#for::transpile(t, node),
      Foreach(_) => foreach::transpile(t, node),
      Function(_) => function::transpile(t, node),
      FunctionCall(_) => function_call::transpile(t, node),
      If(_) => r#if::transpile(t, node),
      Source(_) => source::transpile(t, node),
      Value(_) => value::transpile(t, node),
      While(_) => r#while::transpile(t, node),

      Break => Ok(t.use_indent("break")),
      Continue => Ok(t.use_indent("continue")),
      Return(value) => Ok(t.use_indent(&format!("return {value}"))),

      Empty => Ok(String::new()),
    }?;

    output.push(code);
  }

  Ok(output.join("\n"))
}
