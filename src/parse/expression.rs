use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
  left: Box<Node>,
  operator: Box<Node>,
  right: Box<Node>,
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  println!("ph: {:?}", ph);
  unimplemented_f!("Not implemented")
}
