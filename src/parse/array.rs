use super::{parse_helper::ParseHelper, value::Value};
use crate::{
  check_token,
  parse::{
    error::{Error, ParserResult},
    value,
  },
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Vec<Value>> {
  check_token!(ph, TT::LBracket);
  ph.advance();

  let mut values = Vec::new();

  if ph.peek(0) == Some(&TT::LBracket) {
    return Ok(values);
  }

  loop {
    values.push(value::parse_inner(ph)?);

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RBracket) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      _ => return Err(Error::end(ph)),
    };
  }

  ph.advance();

  Ok(values)
}
