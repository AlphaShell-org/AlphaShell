use crate::{check_token, types::TokenType};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
  files: Vec<Node>,
}

impl Import {
  pub fn new(files: Vec<Node>) -> Self {
    Self { files }
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TokenType::Import);
  ph.advance();

  let mut files = Vec::new();

  while let Some(param) = ph.peak(0) {
    if let TokenType::String(string) = param {
      files.push(Node::String(string.clone()));
    } else if param == &TokenType::RParen {
      break;
    } else {
      return Err(Error::unexpected(ph.get(0).unwrap()));
    }

    ph.advance();

    if let Some(token) = ph.peak(0) {
      match token {
        TokenType::Comma => ph.advance(),
        TokenType::Semicolon => break,
        _ => return Err(Error::unexpected(ph.get(0).unwrap())),
      }
    } else {
      return Err(Error::end());
    }
  }

  let import = Node::Import(Import::new(files));

  Ok(import)
}
