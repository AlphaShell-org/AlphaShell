use super::{error::ParserResult, node::Node, parse_helper::ParseHelper, value::Value};
use crate::{
  check_token,
  parse::{block, error::Error, value},
  types::TT,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Case {
  pub value: String,
  pub block: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Switch {
  pub arg: Value,
  pub cases: Vec<Case>,
}

impl Switch {
  pub fn new(arg: Value, cases: Vec<Case>) -> Self {
    Self { arg, cases }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  ph.advance();

  let arg = value::parse_inner(ph)?;

  check_token!(ph, TT::LBrace);

  ph.advance();

  let mut cases = vec![];

  loop {
    match ph.peek(0) {
      Some(TT::Case) => (),
      Some(TT::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    let value = match ph.peek(0) {
      Some(TT::String(value) | TT::RawString(value)) => value.clone(),
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    let variables = ph.variables.clone();

    let block = Box::new(block::parse(ph, variables)?);

    cases.push(Case { value, block });
  }

  check_token!(ph, TT::RBrace);

  ph.advance();

  Ok(Node::Switch(Switch::new(arg, cases)))
}
