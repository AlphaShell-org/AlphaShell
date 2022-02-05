use std::path::Path;

use crate::{check_token, types::TT};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  check_token!(ph, TT::Import | TT::Source);

  let static_import = ph.peek(0) == Some(&TT::Import);

  ph.advance();

  let mut files = Vec::new();

  loop {
    match ph.peek(0) {
      Some(TT::String(string)) => files.push(string.clone()),
      Some(TT::Semicolon) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TT::Comma) => ph.advance(),
      Some(TT::Semicolon) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };
  }

  ph.advance();

  if static_import {
    let mut imported_trees = vec![];
    for file in files {
      let contents = crate::read_file(Path::new(&file));

      let tokens = match crate::tokenize(&contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(Error::new(&e.msg, None)),
      };

      let tree = match crate::parse(&tokens) {
        Ok(tree) => tree,
        Err(e) => return Err(e),
      };

      imported_trees.push(Node::Block(tree));
    }

    Ok(Node::ImportedCode(imported_trees))
  } else {
    let import = Node::Import(files);

    Ok(import)
  }
}
