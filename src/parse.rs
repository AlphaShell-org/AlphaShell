mod error;
mod node;
mod parse_helper;
mod utils;

use error::{Error, ParserResult};
use node::Node;
use parse_helper::ParseHelper;

mod array;
mod block;
mod expression;
mod r#for;
mod function;
mod function_call;
mod r#if;
mod import;
mod map;
mod r#return;
mod var;
mod r#while;

#[allow(clippy::enum_glob_use)]
use crate::types::{Token, TokenType::*};

pub fn parse(tokens: &[Token]) -> ParserResult<Vec<Node>> {
  println!("Parsing!");

  let mut ph = ParseHelper::new(tokens.to_vec());

  while let Some(token) = ph.peek(0) {
    let node = match token {
      Import => import::parse(&mut ph),
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
            _ => expression::parse(&mut ph),
          }
        } else {
          return Err(Error::end());
        }
      }

      _ => {
        return Err(Error::new(
          &f!("Token {token} not yet implemented"),
          ph.get(0),
        ))
      }
    };

    ph.push_tree(node?);

    ph.advance();
  }

  println!("Done parsing");

  Ok(ph.get_tree())
}
