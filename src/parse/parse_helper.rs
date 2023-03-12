use std::collections::HashSet;

use crate::types::{Token, TokenType};

#[derive(Debug)]
pub struct ParseHelper {
  tokens: Vec<Token>,
  index: usize,
  pub variables: HashSet<String>,
  pub exports: HashSet<String>,
}

impl ParseHelper {
  pub fn new(tokens: Vec<Token>, variables: HashSet<String>) -> ParseHelper {
    ParseHelper {
      tokens,
      index: 0,
      variables,
      exports: HashSet::new(),
    }
  }

  pub fn get(&self, offset: isize) -> Option<&Token> {
    let i = self.index.saturating_add_signed(offset);

    self.tokens.get(i)
  }

  pub fn peek(&self, offset: isize) -> Option<&TokenType> {
    let i = self.index.saturating_add_signed(offset);

    self.tokens.get(i).map(|token| &token.r#type)
  }

  pub fn get_tokens(&self) -> &Vec<Token> {
    &self.tokens
  }

  #[cfg(debug_assertions)]
  pub fn get_index(&self) -> usize {
    self.index
  }

  #[cfg(debug_assertions)]
  pub fn pretty_print_tokens(&self) -> String {
    self
      .tokens
      .iter()
      .enumerate()
      .map(|(i, token)| format!("{i}: {token}"))
      .collect::<Vec<_>>()
      .join(",\n")
  }

  pub fn advance(&mut self) {
    self.index += 1;
  }
}
