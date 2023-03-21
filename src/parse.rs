mod error;
pub mod node;
mod parse_helper;
mod utils;

use std::collections::HashSet;

use error::{Error, ParserResult};
use node::Node;
use parse_helper::ParseHelper;

pub mod array;
pub mod block;
pub mod declaration;
pub mod expression;
pub mod external;
pub mod r#for;
pub mod function;
pub mod function_call;
pub mod r#if;
pub mod import;
pub mod map;
pub mod r#return;
pub mod value;
pub mod r#while;

use crate::{
  check_token,
  types::{
    Token,
    TokenType::{
      Break, Continue, Dollar, Export, External, For, Function, Identifier, If, Import, LParen,
      Let, Return, Source, While,
    },
    TT,
  },
};

pub fn parse(tokens: &[Token]) -> ParserResult<Vec<Node>> {
  let variables = (0..10).map(|x| x.to_string()).collect();

  inner(tokens, variables).map(|(nodes, _)| nodes)
}

macro_rules! simple_token {
  ($ph:expr, $tok:expr) => {{
    $ph.advance();
    check_token!(&$ph, TT::Semicolon);
    $ph.advance();
    Ok($tok)
  }};
}

pub fn inner(
  tokens: &[Token],
  variables: HashSet<String>,
) -> ParserResult<(Vec<Node>, HashSet<String>)> {
  let mut ph = ParseHelper::new(tokens.to_vec(), variables);

  let mut tree = vec![];

  while let Some(token) = ph.peek(0) {
    let node = match token {
      Import | Source => {
        let nodes = import::parse(&mut ph)?;
        tree.extend_from_slice(&nodes);

        Ok(Node::Empty)
      }
      Function => r#function::parse(&mut ph),
      Export | Let => declaration::parse(&mut ph),
      For => r#for::parse(&mut ph),
      While => r#while::parse(&mut ph),
      If => r#if::parse(&mut ph),
      Return => r#return::parse(&mut ph),
      Continue => simple_token!(ph, Node::Continue),
      Break => simple_token!(ph, Node::Break),
      External => external::parse(&mut ph),

      Identifier(..) | Dollar => {
        if let Some(next) = ph.peek(1) {
          match next {
            LParen => function_call::parse(&mut ph),
            _ => expression::parse(&mut ph),
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

  Ok((tree, ph.exports))
}
