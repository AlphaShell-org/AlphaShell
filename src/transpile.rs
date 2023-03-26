mod error;
mod transpiler;
// mod utils;

use error::TranspileResult;

use self::transpiler::Transpiler;
use crate::parse::node::Node::{
  self, Block, Break, Continue, Declaration, Empty, Expression, For, Foreach, Function,
  FunctionCall, If, IfLet, Return, Source, Switch, Value, While, WhileLet,
};

mod block;
mod declaration;
mod expression;
mod r#for;
mod foreach;
mod function;
mod function_call;
mod r#if;
mod r#return;
mod source;
mod switch;
mod value;
mod r#while;

mod utils;

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
      Expression(_) => expression::transpile(t, node),
      For(_) => r#for::transpile(t, node),
      Foreach(_) => foreach::transpile(t, node),
      Function(_) => function::transpile(t, node),
      FunctionCall(_) => function_call::transpile(t, node),
      If(_) => r#if::transpile(t, node),
      IfLet(_) => r#if::transpile_let(t, node),
      Source(_) => source::transpile(t, node),
      Value(_) => value::transpile(t, node),
      While(_) => r#while::transpile(t, node),
      WhileLet(_) => r#while::transpile_let(t, node),
      Return(_) => r#return::transpile(t, node),

      Break => Ok(t.use_indent("break")),
      Continue => Ok(t.use_indent("continue")),

      Switch(_) => switch::transpile(t, node),

      Empty => Ok(String::new()),
    }?;

    output.push(code);
  }

  Ok(output.join("\n"))
}
