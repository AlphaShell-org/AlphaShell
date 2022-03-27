use super::{
  error::{Error, ParserResult},
  parse_helper::ParseHelper,
  value,
  value::Value,
};
use crate::types::{TokenType, TT};

#[derive(Debug, PartialEq, Clone)]
enum ConditionValue {
  Equal(Value, Value),
  NotEqual(Value, Value),
  Greater(Value, Value),
  Less(Value, Value),
  GreaterEqual(Value, Value),
  LessEqual(Value, Value),
  RegexMatch(Value, Value),
  And(Box<Condition>, Box<Condition>),
  Or(Box<Condition>, Box<Condition>),
  Not(Value),
}

fn is_conditional_operator(token: TokenType) -> bool {
  match token {
    TokenType::And => true,
    TokenType::Or => true,
    TokenType::Not => true,
    TokenType::Equal => true,
    TokenType::Less => true,
    TokenType::Greater => true,
    TokenType::LessEqual => true,
    TokenType::GreaterEqual => true,
    TokenType::NotEqual => true,
    _ => false,
  }
}

static PRECEDENCE: phf::Map<TokenType, u8> = phf::phf_map! {
  TokenType::And => 1,
    TokenType::Or => 2,
    TokenType::Not => 3,
    TokenType::Equal => true,
    TokenType::Less => true,
    TokenType::Greater => true,
    TokenType::LessEqual => true,
    TokenType::GreaterEqual => true,
    TokenType::NotEqual => true,
};
fn operator_precedence(token: TokenType) -> u8 {}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
  Value(Value),
  Compound(ConditionValue),
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Condition> {
  let tokens = vec![];

  let level = 0;

  while let Some(token) = ph.peak(0) {
    if token == TT::LBrace {
      break;
    }

    tokens.push(token);

    ph.advance();
  }

  if tokens.is_empty() {
    return Err(Error::new("Missing condition in if statement", ph.get(0)));
  }

  let stack = vec![];
  let rpn = vec![];

  while let Some(token) = ph.get(0) {
    let token = token.clone();

    ph.advance();
  }

  todo!();

  // Ok(condition)
}
