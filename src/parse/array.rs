use anyhow::Result;

use super::{parse_helper::ParseHelper, value::Value};
use crate::{
  check_token,
  parse::{error, value},
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> Result<Vec<Value>> {
  check_token!(ph, TT::LBracket);
  ph.advance();

  let mut values = Vec::new();

  if ph.peek(0) == Some(&TT::RBracket) {
    ph.advance();
    return Ok(values);
  }

  loop {
    let value = value::parse_inner(ph).context("Parsing array value")?;
    values.push(value);

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RBracket) => break,
      Some(_) => return Err(error::unexpected(ph)),
      _ => return Err(error::end(ph)),
    };
  }

  ph.advance();

  Ok(values)
}
