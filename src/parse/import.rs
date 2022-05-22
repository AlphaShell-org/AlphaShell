use std::{fs::File, io::Read, path::Path};

use super::{
  error::{Error, ParserResult},
  node::Node,
  parse_helper::ParseHelper,
};
use crate::{
  check_token,
  types::{Token, TT},
};

fn read_file(path: &Path, token: &Token) -> Result<String, Error> {
  let mut file = match File::open(path) {
    Ok(path) => path,
    Err(e) => {
      return Err(Error::new(
        &format!("Couldn't open file '{path:?}', error: '{e}'"),
        Some(token),
      ))
    }
  };

  let mut contents = String::new();

  match file.read_to_string(&mut contents) {
    Ok(_) => Ok(contents),
    Err(e) => Err(Error::new(
      &format!("Error reading file '{path:?}', error: '{e}'"),
      Some(token),
    )),
  }
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Vec<Node>> {
  check_token!(ph, TT::Import | TT::Source);

  let static_import = ph.peek(0) == Some(&TT::Import);
  let token = ph.get(0).cloned().unwrap();

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
    let mut imported = vec![];
    for file in files {
      let contents = read_file(Path::new(&file), &token)?;

      let tokens = match crate::tokenize(&contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(Error::new(&e.msg, None)),
      };

      let tree = match crate::parse(&tokens) {
        Ok(tree) => tree,
        Err(e) => return Err(e),
      };

      imported.extend_from_slice(&tree);
    }

    Ok(imported)
  } else {
    let import = files.into_iter().map(Node::Source).collect();
    Ok(import)
  }
}
