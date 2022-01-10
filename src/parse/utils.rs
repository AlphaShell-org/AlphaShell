use crate::types::Token;

use super::parse_helper::ParseHelper;

#[macro_export]
macro_rules! check_token {
  ($ph:ident, $(|)? $( $pattern:pat_param )|+ ) => {
    if let Some(token) = $ph.peak(0) {

      let valid = match token {
        $( $pattern )|+  => false,
        _ => true
      };
      if valid {
        return Err(Error::unexpected($ph.get(0).unwrap()));
      }
    } else {
      return Err(Error::new("Unexpected end of input", $ph.get(0)));
    }
  };
}
