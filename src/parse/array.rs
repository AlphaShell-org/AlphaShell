use super::{
  parse_helper::ParseHelper,
  value::{Value, Value},
};
use crate::{
  check_token,
  parse::error::{Error, ParserResult},
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Value> {
  check_token!(ph, TT::LBracket);
  ph.advance();

  let mut values = Vec::new();

  if ph.peek(0) == Some(&TT::LBracket) {
    return Ok(Value::Raw(Value::Array(values)));
  }

  loop {
    match ph.peek(0) {
      Some(TT::String(value)) => values.push(value.clone()),
      Some(_) => return Err(Error::unexpected(ph)),
      _ => return Err(Error::end(ph)),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RBracket) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      _ => return Err(Error::end(ph)),
    };
  }

  Ok(Value::Raw(Value::Array(values)))
}
