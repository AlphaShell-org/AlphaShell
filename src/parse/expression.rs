use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
  value,
};
use crate::{check_token, types::TT};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  let value = value::parse_inner(ph)?;

  check_token!(ph, TT::Semicolon);
  ph.advance();

  Ok(Node::Expression(value))
}
