use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};
use crate::{check_token, types::TT};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
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
      let body = super::parse(&tmp)?;
      let block = Node::Block(body);

      ph.advance();

      return Ok(block);
    }

    tmp.push(token.clone());

    ph.advance();
  }

  Err(Error::end(ph))
}
