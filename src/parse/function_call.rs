use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value::{Literal, Value},
};
use crate::{check_token, parse::value, types::TT};

#[derive(Debug, PartialEq, Clone)]
pub enum Next {
  Call(Box<FunctionCall>),
  File(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
  pub name: String,
  pub args: Vec<Value>,
  pub is_daemon: bool,
  pub next: Option<Next>,
}

impl FunctionCall {
  pub fn new(name: String, args: Vec<Value>, next: Option<Next>, is_daemon: bool) -> Self {
    Self {
      name,
      args,
      is_daemon,
      next,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
enum FType {
  Call,
  Aritmetics,
  String(String),
  RawString(String),
}

pub fn parse_inner(ph: &mut ParseHelper) -> ParserResult<FunctionCall> {
  let (name, type_) = if let Some(token) = ph.peek(0) {
    match token {
      TT::Identifier(name) => (name.clone(), FType::Call),
      TT::String(string) => ("echo".to_owned(), FType::String(string.clone())),
      TT::RawString(string) => ("echo".to_owned(), FType::RawString(string.clone())),
      TT::Dollar => ("$".to_string(), FType::Aritmetics),
      _ => return Err(Error::unexpected(ph)),
    }
  } else {
    return Err(Error::end(ph));
  };

  ph.advance();

  let args = match &type_ {
    FType::String(string) => vec![Value::Literal(Literal::String(string.clone()))],
    FType::RawString(string) => vec![Value::Literal(Literal::RawString(string.clone()))],
    _ => parse_args(ph)?,
  };

  let next = if let Some(TT::Pipe) = ph.peek(0) {
    ph.advance();

    if let Some(TT::String(string)) = ph.peek(0) {
      Some(Next::File(string.clone()))
    } else {
      Some(Next::Call(Box::new(parse_inner(ph)?)))
    }
  } else {
    None
  };

  let is_daemon = if matches!(ph.peek(0), Some(TT::Daemon)) {
    ph.advance();
    true
  } else {
    false
  };

  if type_ == FType::Aritmetics && next.is_some() {
    let new = FunctionCall {
      name: "echo".to_owned(),
      args: vec![Value::FunctionCall(FunctionCall::new(
        name, args, None, is_daemon,
      ))],
      is_daemon,
      next,
    };

    return Ok(new);
  }

  Ok(FunctionCall::new(name, args, next, is_daemon))
}

fn parse_args(ph: &mut ParseHelper) -> Result<Vec<Value>, Error> {
  check_token!(ph, TT::LParen);

  ph.advance();

  let mut args = vec![];
  while let Some(arg) = ph.peek(0) {
    if arg == &TT::RParen {
      break;
    }

    let arg = if let Some(TT::Flag(str)) = ph.peek(0).cloned() {
      ph.advance();
      Value::Literal(Literal::RawString(str))
    } else {
      value::parse_inner(ph)?
    };

    args.push(arg);

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RParen) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  check_token!(ph, TT::RParen);

  ph.advance();

  Ok(args)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Identifier(..) | TT::Dollar);

  let fn_call = parse_inner(ph)?;

  check_token!(ph, TT::Semicolon);
  ph.advance();

  Ok(Node::FunctionCall(fn_call))
}
