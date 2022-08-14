use super::{
  error::{Error, ParserResult},
  parse_helper::ParseHelper,
};
use crate::{check_token, parse::node::Node, types::TT};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::External);

  ph.advance();

  match ph.peek(0) {
    Some(TT::String(string)) => ph.variables.insert(string.clone()),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  loop {
    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::Semicolon) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TT::String(string)) => ph.variables.insert(string.clone()),
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  ph.advance();

  Ok(Node::Empty)
}
