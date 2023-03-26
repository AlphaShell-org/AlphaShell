use super::{error::ParserResult, parse_helper::ParseHelper, value::Value};
use crate::{
  check_token,
  parse::{error::Error, value},
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Vec<(String, Value)>> {
  check_token!(ph, TT::LBrace);
  ph.advance();

  let mut items = Vec::new();
  loop {
    let key = match ph.peek(0) {
      Some(TT::Identifier(key) | TT::String(key) | TT::RawString(key)) => key.clone(),
      Some(TT::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    check_token!(ph, TT::Colon);

    ph.advance();

    let value = value::parse_inner(ph)?;

    items.push((key, value));

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  ph.advance();

  Ok(items)
}
