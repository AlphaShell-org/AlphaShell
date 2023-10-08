use std::collections::HashSet;

use anyhow::Result;

use super::{node::Node, parse_helper::ParseHelper};
use crate::{check_token, parse::error, types::TT};

pub type Block = Vec<Node>;

pub fn parse_inner(ph: &mut ParseHelper, variables: HashSet<String>) -> Result<Block> {
  check_token!(ph, TT::LBrace);

  ph.advance();

  let mut tmp = Vec::new();
  let mut braces_level = 1;

  while let Some(token) = ph.get(0) {
    match token.r#type {
      TT::LBrace => braces_level += 1,
      TT::RBrace => braces_level -= 1,
      _ => {}
    }

    if braces_level == 0 {
      let (body, exports) = super::inner(&tmp, variables).context("Parsing block")?;

      ph.variables.extend(exports.iter().cloned());
      ph.exports.extend(exports.iter().cloned());

      ph.advance();

      return Ok(body);
    }

    tmp.push(token.clone());
    ph.advance();
  }

  Err(error::end(ph))
}

pub fn parse(ph: &mut ParseHelper, variables: HashSet<String>) -> Result<Node> {
  let body = parse_inner(ph, variables)?;
  let block = Node::Block(body);
  Ok(block)
}
