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

  let token = ph.get(0).cloned().unwrap();

  ph.advance();

  let mut files = Vec::new();

  match ph.peek(0) {
    Some(TT::String(string)) => files.push(string.clone()),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  loop {
    match ph.peek(0) {
      Some(TT::Comma) => {}
      Some(TT::Semicolon) => break,
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();

    match ph.peek(0) {
      Some(TT::String(string)) => files.push(string.clone()),
      Some(_) => return Err(Error::unexpected(ph)),
      None => return Err(Error::end(ph)),
    };

    ph.advance();
  }

  ph.advance();

  if token.r#type == TT::Import {
    macro_rules! unwrap_or_error {
      ($input:expr, $file:ident) => {
        match $input {
          Ok(a) => a,
          Err(e) => {
            return Err(Error::new(
              &format!("Error while importing {}:\n{e}", $file),
              Some(&token),
            ))
          }
        }
      };
    }

    files
      .into_iter()
      .map(|file| {
        let contents = read_file(Path::new(&file), &token)?;

        let tokens = unwrap_or_error!(crate::tokenize(&contents), file);
        let tree = unwrap_or_error!(crate::parse(&tokens), file);

        Ok(tree)
      })
      .collect::<ParserResult<Vec<Vec<Node>>>>()
      .map(|trees| trees.into_iter().flatten().collect())
  } else {
    Ok(files.into_iter().map(Node::Source).collect())
  }
}
