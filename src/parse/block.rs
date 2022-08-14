use std::collections::HashSet;

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};
use crate::{check_token, types::TT};

pub type Block = Vec<Node>;

pub fn parse_inner(ph: &mut ParseHelper, variables: HashSet<String>) -> ParserResult<Block> {
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
      let body = super::inner(&tmp, variables)?;

      ph.advance();

      return Ok(body);
    }

    tmp.push(token.clone());
    ph.advance();
  }

  Err(Error::end(ph))
}

pub fn parse(ph: &mut ParseHelper, variables: HashSet<String>) -> ParserResult<Node> {
  let body = parse_inner(ph, variables)?;
  let block = Node::Block(body);
  Ok(block)
}
