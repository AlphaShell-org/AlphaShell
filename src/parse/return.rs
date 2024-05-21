use super::{error::ParserResult, node::Node, parse_helper::ParseHelper};
use crate::{
  check_token,
  parse::{error::Error, value},
  types::TT,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Return);
  ph.advance();

  let value = if let Some(TT::Integer(int)) = ph.peek(0) {
    if !(0..256).contains(int) {
      return Err(Error::new(
        &format!("Invalid return value: {int}"),
        ph.get(0),
      ));
    }

    let int = *int;

    ph.advance();

    Node::Return(value::Value::Literal(value::Literal::Int(int)))
  } else {
    let value = value::parse_inner(ph)?;
    Node::Return(value)
  };

  check_token!(ph, TT::Semicolon);

  ph.advance();

  Ok(value)
}
