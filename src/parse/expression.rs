use anyhow::Result;

use super::{node::Node, parse_helper::ParseHelper, value};
use crate::{check_token, types::TT};

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  let value = value::parse_inner(ph).context("Parsing Expression")?;

  check_token!(ph, TT::Semicolon);
  ph.advance();

  Ok(Node::Expression(value))
}
