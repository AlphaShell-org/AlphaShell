use super::{
  error::ParserResult,
  node::Node,
  parse_helper::ParseHelper,
  value::{Data, Value},
};
use crate::{check_token, parse::error::Error, types::TT};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::LBrace);
  ph.advance();

  let mut items = Vec::new();
  loop {
    let key = match ph.peek(0) {
      Some(TT::String(key)) => key.clone(),
      Some(TT::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    check_token!(ph, TT::Colon);

    ph.advance();

    let value = match ph.peek(0) {
      Some(TT::String(value)) => value.clone(),
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    items.push((key, value));

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::RBrace) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  ph.advance();

  Ok(Node::Value(Value::Raw(Data::Map(items))))
}
