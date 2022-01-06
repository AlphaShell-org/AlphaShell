mod error;
mod node;
mod parse_helper;
mod utils;

use error::{Error, Result};
use node::Node;
use parse_helper::ParseHelper;

mod array;
mod block;
mod expression;
mod flow;
mod r#for;
mod function;
mod r#if;
mod import;
mod map;
mod number;
mod simple;
mod string;
mod var;
mod r#while;

#[allow(clippy::enum_glob_use)]
use crate::types::{Token, TokenType::*};

pub fn parse(tokens: &[Token]) -> Result<Vec<Node>> {
  let mut ph = ParseHelper::new(tokens.to_vec());

  while let Some(token) = ph.peak(0) {
    let node = match token {
      Import => import::parse(&mut ph),
      Function => r#function::parse(&mut ph),
      Export | Let => var::parse(&mut ph),
      For => r#for::parse(&mut ph),
      While => r#while::parse(&mut ph),
      If => r#if::parse(&mut ph),
      Return | Continue | Break => flow::parse(&mut ph),

      _ => return Err(Error::new("Token {token} not yet implemented", ph.get(0))),
    };

    ph.push_tree(node?);

    ph.advance();
  }

  Ok(ph.get_tree())
}
