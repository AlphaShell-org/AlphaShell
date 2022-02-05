#[macro_export]
macro_rules! check_token {
  ($ph:ident, $(|)? $( $pattern:pat_param )|+ ) => {
    match $ph.peek(0) {
      Some($( $pattern )|+)  => {},
      Some(_) => return Err(Error::unexpected($ph)),
      _ => return Err(Error::end($ph))
    }
  };
}
