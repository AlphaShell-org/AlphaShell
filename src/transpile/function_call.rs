use super::{
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
  value,
};
use crate::parse::{
  function_call::{FunctionCall, Next},
  node::Node,
};

fn transpile_next(t: &mut Transpiler, next: &Next) -> TranspileResult<String> {
  match next {
    Next::Call(node) => transpile(t, node),
    Next::File(file) => Ok(format!(">{file}")),
  }
}

pub fn transpile_inner(t: &mut Transpiler, call: &FunctionCall) -> TranspileResult<String> {
  let FunctionCall {
    name,
    args,
    is_daemon,
    next,
  } = call;

  let basic_call = if args.is_empty() {
    name.clone()
  } else {
    let mut transpiled_args = vec![];
    for arg in args {
      transpiled_args.push(value::transpile(t, arg)?);
    }
    let args = transpiled_args.join(" ");

    format!("{name} {args}")
  };

  let mut call = if let Some(next) = next {
    let next = transpile_next(t, next)?;
    format!("{basic_call} | {next}")
  } else if *is_daemon {
    format!("{basic_call} &")
  } else {
    basic_call
  };

  if t.get_block() != &Some(BlockType::Expression) {
    call = t.use_indent(&call);
  }

  Ok(call)
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::FunctionCall(call) => transpile_inner(t, call),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
