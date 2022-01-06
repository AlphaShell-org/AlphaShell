use crate::types::{Token, TokenType};

use super::node::Node;

#[derive(Debug)]
pub struct ParseHelper {
  tokens: Vec<Token>,
  tree: Vec<Node>,
  index: usize,
}

impl ParseHelper {
  pub fn new(tokens: Vec<Token>) -> ParseHelper {
    ParseHelper {
      tokens,
      tree: Vec::new(),
      index: 0,
    }
  }

  pub fn get(&self, offset: i32) -> Option<&Token> {
    let i = self.index as i32 + offset;

    if i < 0 {
      return None;
    }

    self.tokens.get(i as usize)
  }

  pub fn peak(&self, offset: i32) -> Option<&TokenType> {
    let i = self.index as i32 + offset;

    if i < 0 {
      return None;
    }

    self.tokens.get(i as usize).map(|token| &token.r#type)
  }

  pub fn peak_tree(&self, offset: i32) -> Option<&Node> {
    let i = self.tree.len() as i32 - offset;

    if i < 0 {
      return None;
    }

    self.tree.get(i as usize)
  }

  pub fn push_tree(&mut self, value: Node) {
    self.tree.push(value);
  }

  pub fn get_tree(&mut self) -> Vec<Node> {
    std::mem::take(&mut self.tree)
  }

  pub fn advance(&mut self) {
    self.index += 1;
  }
}
