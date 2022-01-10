use crate::{check_token, parse::expression, types::TokenType};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
  name: String,
  args: Vec<Node>,
  is_daemon: bool,
  next: Option<Box<Node>>,
}

impl FunctionCall {
  pub fn new(name: String, args: Vec<Node>, next: Option<Box<Node>>, is_daemon: bool) -> Self {
    Self {
      name,
      args,
      is_daemon,
      next,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::Identifier(..));

  let name = if let Some(token) = ph.peak(0) {
    match token {
      TokenType::Identifier(name) => name.clone(),
      _ => return Err(Error::unexpected(ph.get(0).unwrap())),
    }
  } else {
    return Err(Error::end());
  };

  ph.advance();

  check_token!(ph, TokenType::LParen);

  ph.advance();

  let mut args = vec![];

  while let Some(arg) = ph.peak(0) {
    if arg == &TokenType::RParen {
      break;
    }

    let arg = expression::parse(ph)?;

    args.push(arg);

    if let Some(token) = ph.peak(0) {
      match token {
        TokenType::Comma => ph.advance(),
        TokenType::RParen => break,
        _ => return Err(Error::unexpected(ph.get(0).unwrap())),
      }
    } else {
      return Err(Error::end());
    }
  }

  check_token!(ph, TokenType::RParen);

  ph.advance();

  let next = if let Some(TokenType::Pipe) = ph.peak(0) {
    ph.advance();

    if let Some(TokenType::String(string)) = ph.peak(0) {
      Some(Box::new(Node::String(string.clone()))) // redirect to file
    } else {
      Some(Box::new(parse(ph)?))
    }
  } else {
    None
  };

  let is_daemon = matches!(ph.peak(0), Some(TokenType::Daemon));

  if is_daemon {
    ph.advance();
  }

  if next.is_none() {
    check_token!(ph, TokenType::Semicolon);
    ph.advance();
  }

  let function_call = FunctionCall::new(name, args, next, is_daemon);

  Ok(Node::FunctionCall(function_call))
}
