use crate::types::{Token, TokenType};

use super::{
  error::{Error, Result},
  parse_helper::ParseHelper,
};

pub fn check_token(ph: &ParseHelper, valid: &[TokenType]) -> Result<()> {
  if let Some(token) = ph.peak(1) {
    if !valid.contains(token) {
      return Err(Error::unexpected(ph.get(1).unwrap()));
    }
  } else {
    return Err(Error::new("Unexpected end of input", ph.get(1)));
  }

  Ok(())
}

pub fn load_until_closing(ph: &mut ParseHelper) -> Vec<Token> {
  unimplemented!()
}
