use super::{
  error::{Error, Result},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
  pub value: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Int {
  pub value: i64,
}

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  println!("ph: {:?}", ph);
  unimplemented_f!("Not implemented")
}
