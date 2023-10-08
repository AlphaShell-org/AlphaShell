#[macro_export]
macro_rules! check_token {
  ($ph:expr, $(|)? $( $pattern:pat_param )|+ ) => {
    match $ph.peek(0) {
      Some($( $pattern )|+)  => {},
      Some(_) => return Err($crate::parse::error::unexpected($ph)),
      _ => return Err($crate::parse::error::end($ph))
    }
  };
}
