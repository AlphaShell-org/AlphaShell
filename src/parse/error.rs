use anyhow::{anyhow, Error};

use super::parse_helper::ParseHelper;

pub fn unexpected(ph: &ParseHelper) -> Error {
  let token = ph.get(0).unwrap();
  anyhow!("Unexpected token {token}")
}

// pub fn duplicate_variable(ph: &ParseHelper) -> Self {
//   let token = ph.get(0).unwrap();
//
//   #[cfg(debug_assertions)]
//   println!(
//     "{}\ncurrent index: {}",
//     ph.pretty_print_tokens(),
//     ph.get_index()
//   );
//   Self::new(&format!("Duplicate variable {token}"), Some(token))
// }

pub fn undefined_variable(ph: &ParseHelper) -> Error {
  let token = ph.get(0).unwrap();
  anyhow!("Undefined variable {token}")
}

pub fn end(ph: &ParseHelper) -> Error {
  let last = ph.get_tokens().last().unwrap();

  anyhow!("Unexpected end of input after {last}")
}
