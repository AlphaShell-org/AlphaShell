use super::{
  error::{Error, TranspileResult},
  value,
};
use crate::parse::{
  function_call::{FunctionCall, Next},
  node::Node,
};

fn transpile_next(next: &Next) -> TranspileResult<String> {
  match next {
    Next::Call(node) => transpile(node),
    Next::File(file) => Ok(format!(">{file}")),
  }
}

pub fn transpile_inner(call: &FunctionCall) -> TranspileResult<String> {
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
      transpiled_args.push(value::transpile(arg)?);
    }
    let args = transpiled_args.join(" ");

    format!("{name} {args}")
  };

  let call = if let Some(next) = next {
    let next = transpile_next(next)?;
    format!("{basic_call} | {next}")
  } else if *is_daemon {
    format!("{basic_call} &")
  } else {
    basic_call
  };

  Ok(call)
}

pub fn transpile(node: &Node) -> TranspileResult<String> {
  match node {
    Node::FunctionCall(call) => transpile_inner(call),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
