mod error;
pub mod node;
mod parse_helper;
mod utils;

use error::{Error, ParserResult};
use node::Node;
use parse_helper::ParseHelper;

pub mod array;
pub mod block;
pub mod r#for;
pub mod function;
pub mod function_call;
pub mod r#if;
pub mod import;
pub mod map;
pub mod r#return;
pub mod value;
pub mod var;
pub mod r#while;

#[allow(clippy::enum_glob_use)]
use crate::types::{Token, TokenType::*};

pub fn parse(tokens: &[Token]) -> ParserResult<Vec<Node>> {
  let mut ph = ParseHelper::new(tokens.to_vec());

  let mut tree = vec![];

  while let Some(token) = ph.peek(0) {
    let node = match token {
      Import | Source => {
        let nodes = import::parse(&mut ph)?;
        tree.extend_from_slice(&nodes);

        Ok(Node::Empty)
      }
      Function => r#function::parse(&mut ph),
      Export | Let => var::parse(&mut ph),
      For => r#for::parse(&mut ph),
      While => r#while::parse(&mut ph),
      If => r#if::parse(&mut ph),
      Return => r#return::parse(&mut ph),
      Continue => Ok(Node::Continue),
      Break => Ok(Node::Break),

      Identifier(..) => {
        if let Some(next) = ph.peek(1) {
          match next {
            LParen => function_call::parse(&mut ph),
            _ => value::parse(&mut ph),
          }
        } else {
          return Err(Error::end(&ph));
        }
      }
      _ => return Err(Error::unexpected(&ph)),
    };

    match node? {
      Node::Empty => {}
      node => tree.push(node),
    }
  }

  Ok(tree)
}
