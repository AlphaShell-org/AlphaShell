mod error;
use error::{Error, Result};

mod node;
use node::Node;

mod statement;
use statement::Statement;

mod import;
use import::parse_import;

mod expression;
use expression::parse_expression;

mod simple;

mod general;

use super::tokenize::{Token, TokenType};

pub struct ParseTree {
    tree: Vec<Statement>,
}

impl std::fmt::Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tree)
    }
}

struct ParseHelper {
    tokens: Vec<Token>,
    index: usize,
}

impl ParseHelper {
    pub fn new(tokens: Vec<Token>) -> ParseHelper {
        ParseHelper { tokens, index: 0 }
    }

    pub fn token(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn peak(&self, offset: i32) -> Option<TokenType> {
        self.tokens
            .get((self.index as i32 + offset) as usize)
            .map(|token| token.0)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ParseTree> {
    let mut tree: Vec<Box<dyn Node>> = Vec::new();
    let mut stack = Vec::new();

    let mut ph = ParseHelper::new(tokens);

    while let Some(token_type) = ph.peak(0) {
        /* if let Some(last) = stack.last() {
          if token.0 != *last {
            return Err(Error::new(
              &f!("Can't have {token.1} in {last} block"),
              token.2,
            ));
          }
        } */

        let res = if token_type == TokenType::Import {
            parse_import(&mut ph)?
        } else if token_type == TokenType::Function {
            parse_function(ph)?
        } else if token_type == TokenType::If {
            parse_if(ph, stack)
        } else if token_type == TokenType::For {
            if let Some(next) = ph.peak(1) {
                if let Some(next2) = ph.peak(2) {
                    if next == TokenType::Identifier && next2 == TokenType::In {
                        parse_foreach(ph, stack)
                    }
                }
            }

            parse_for(ph, stack)
        } else if token_type == TokenType::Return {
            if !stack.contains(&Grammar::Function) {
                return Err(Error::new(
                    "keyword 'return' can only be used inside a function",
                    ph.token(),
                ));
            }
            parse_return(ph)
        } else if token_type == TokenType::Continue {
            // TODO: convert two single search
            if !stack.contains(&Grammar::For) && !stack.contains(&Grammar::Foreach) {
                return Err(Error::new(
                    "keyword 'continue' can only be used inside a loop",
                    ph.token(),
                ));
            }
            //stmt_continue_t( ph->tok_ctr() );
            ph.advance();
        } else if token_type == TokenType::Break {
            if !stack.contains(&Grammar::For) && !stack.contains(&Grammar::Foreach) {
                return Err(Error::new(
                    "keyword 'break' can only be used inside a loop",
                    ph.token(),
                ));
            }
            //stmt_break_t( ph->tok_ctr() );
            ph.advance();
        } else if token_type == TokenType::LBrace {
            // simple block
            parse_block(ph, stack)?;
        } else {
            // just expressions remain
            parse_expression(&mut ph)?;
            // return Err(Error::new( "failed parsing expression", ph.token().unwrap().pos() );
        };

        tree.push(Box::new(res));

        ph.advance();
    }

    Ok(ParseTree { tree })
}
