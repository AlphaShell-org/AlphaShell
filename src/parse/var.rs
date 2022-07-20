use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value,
};
use crate::{check_token, types::TT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeclarationType {
  Let,
  Export,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
  pub r#type: DeclarationType,
  pub name: String,
  pub value: Box<Node>,
}

impl Declaration {
  pub fn new(r#type: DeclarationType, name: String, value: Box<Node>) -> Self {
    Self {
      r#type,
      name,
      value,
    }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Let | TT::Export);

  let r#type = match ph.peek(0) {
    Some(TT::Let) => DeclarationType::Let,
    Some(TT::Export) => DeclarationType::Export,
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  let name = match ph.peek(0) {
    Some(TT::Identifier(name)) => name.clone(),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  check_token!(ph, TT::Assignment);

  ph.advance();

  let value = value::parse(ph)?;

  check_token!(ph, TT::Semicolon);

  ph.advance();

  let declaration = Node::Declaration(Declaration::new(r#type, name, Box::new(value)));

  Ok(declaration)
}
