use crate::{check_token, parse::value, types::TT};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Next {
  Call(Box<Node>),
  File(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
  pub name: String,
  pub args: Vec<Node>,
  pub is_daemon: bool,
  pub next: Option<Next>,
}

impl FunctionCall {
  pub fn new(name: String, args: Vec<Node>, next: Option<Next>, is_daemon: bool) -> Self {
    Self {
      name,
      args,
      is_daemon,
      next,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Identifier(..));

  let name = if let Some(token) = ph.peek(0) {
    match token {
      TT::Identifier(name) => name.clone(),
      _ => return Err(Error::unexpected(ph)),
    }
  } else {
    return Err(Error::end(ph));
  };

  ph.advance();

  check_token!(ph, TT::LParen);

  ph.advance();

  let mut args = vec![];

  while let Some(arg) = ph.peek(0) {
    if arg == &TT::RParen {
      break;
    }

    let arg = value::parse(ph)?;

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

  let next = if let Some(TT::Pipe) = ph.peek(0) {
    ph.advance();

    if let Some(TT::String(string)) = ph.peek(0) {
      Some(Next::File(string.clone()))
    } else {
      Some(Next::Call(Box::new(parse(ph)?)))
    }
  } else {
    None
  };

  let is_daemon = matches!(ph.peek(0), Some(TT::Daemon));

  if is_daemon {
    ph.advance();
  }

  if next.is_none() {
    check_token!(ph, TT::Semicolon);
    ph.advance();
  }

  let function_call = FunctionCall::new(name, args, next, is_daemon);

  Ok(Node::FunctionCall(function_call))
}
