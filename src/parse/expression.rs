use super::{error::{Error, Result},node::Node, parse_helper::ParseHelper};

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
  pub left: Option<Box<Node>>,
  pub operator: Option<Box<Node>>,
  pub right: Option<Box<Node>>,
}

pub fn parse(ph: &mut ParseHelper) -> Result<Node> {
  println!("ph: {:?}", ph);
  unimplemented_f!("Not implemented")
}
