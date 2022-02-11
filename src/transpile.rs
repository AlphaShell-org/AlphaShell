mod error;
// mod utils;

use error::TranspileResult;

use crate::parse::node::{Node, Node::*};

mod block;
mod declaration;
mod r#for;
mod foreach;
mod function;
mod function_call;
mod r#if;
mod import;
mod index;
mod value;
mod r#while;

pub fn transpile(tree: &[Node]) -> TranspileResult<String> {
  let mut output = Vec::new();

  for node in tree {
    let code = match node {
      Block(_) => block::transpile(node),
      Declaration(_) => declaration::transpile(node),
      For(_) => r#for::transpile(node),
      Foreach(_) => foreach::transpile(node),
      Function(_) => function::transpile(node),
      FunctionCall(_) => function_call::transpile(node),
      If(_) => r#if::transpile(node),
      IndexCall => index::transpile(node),
      Import(_) => import::transpile(node),
      ImportedCode(code) => transpile(code),
      Value(_) => value::transpile(node),
      While() => r#while::transpile(node),

      Break => Ok("break".into()),
      Continue => Ok("continue".into()),
      Return(value) => Ok(format!("return {value}")),
    }?;

    output.push(code);
  }

  Ok(output.join("\n"))
}
