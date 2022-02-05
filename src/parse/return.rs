use crate::{check_token, parse::error::Error, types::TT};

use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Return);
  ph.advance();

  let value = if let Some(TT::Integer(int)) = ph.peek(0) {
    if *int < 0 && *int >= 256 {
      return Err(Error::new(&f!("Invalid return value: {int}"), ph.get(0)));
    }

    Node::Return(*int as u8)
  } else {
    Node::Return(0)
  };

  ph.advance();

  check_token!(ph, TT::Semicolon);

  ph.advance();

  Ok(value)
}
