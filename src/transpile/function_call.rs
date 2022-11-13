use super::{
  error::{Error, TranspileResult},
  transpiler::{BlockType, Transpiler},
  value,
};
use crate::parse::{
  function_call::{FunctionCall, Next},
  node::Node,
};

fn transpile_next(t: &mut Transpiler, next: &Next, node: &Node) -> TranspileResult<String> {
  match next {
    Next::Call(call) => transpile_inner(t, call, node),
    Next::File(file) => Ok(format!(">{file}")),
  }
}

pub fn transpile_inner(
  t: &mut Transpiler,
  call: &FunctionCall,
  node: &Node,
) -> TranspileResult<String> {
  let FunctionCall {
    name,
    args,
    is_daemon,
    next,
  } = call;

  if name == "$" {
    t.push_block(BlockType::Arithmetics);

    let mut transpiled_args = Vec::new();

    for arg in args {
      transpiled_args.push(value::transpile_inner(t, arg, node)?);
    }

    t.pop_block();

    let args = transpiled_args.join(" ");

    if matches!(t.get_block(), Some(BlockType::Generic)) {
      return Ok(t.use_indent(&format!("(( {args} ))")));
    }

    return Ok(format!("$(( {} ))", args));
  }

  let basic_call = if args.is_empty() {
    name.clone()
  } else {
    let mut transpiled_args = vec![];
    for arg in args {
      transpiled_args.push(value::transpile_inner(t, arg, node)?);
    }
    let args = transpiled_args.join(" ");

    format!("{name} {args}")
  };

  let call = if let Some(next) = next {
    t.push_block(BlockType::FunctionCall);
    let next = transpile_next(t, next, node)?;
    t.pop_block();
    format!("{basic_call} | {next}")
  } else if *is_daemon {
    format!("{basic_call} &")
  } else {
    basic_call
  };

  let call = match t.get_block() {
    Some(BlockType::FunctionCall) => call,
    Some(BlockType::Expression) => format!(r#""$({})""#, call),
    _ => t.use_indent(&call),
  };

  Ok(call)
}

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  if let Node::FunctionCall(call) = node {
    transpile_inner(t, call, node)
  } else {
    Err(Error::invalid(node))
  }
}
