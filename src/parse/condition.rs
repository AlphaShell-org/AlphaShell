use super::{
  error::{Error, ParserResult},
  parse_helper::ParseHelper,
  value,
  value::Value,
};
use crate::types::{TokenType, TT};

#[derive(Debug, PartialEq, Clone)]
enum ConditionalOperator {
  Equal,
  NotEqual,
  Greater,
  Less,
  GreaterEqual,
  LessEqual,
  RegexMatch,
}

impl TryFrom<&TokenType> for ConditionalOperator {
  type Error = ();

  fn try_from(value: &TokenType) -> Result<Self, Self::Error> {
    match value {
      TT::Equal => Ok(Self::Equal),
      TT::NotEqual => Ok(Self::NotEqual),
      TT::Greater => Ok(Self::Greater),
      TT::Less => Ok(Self::Less),
      TT::GreaterEqual => Ok(Self::GreaterEqual),
      TT::LessEqual => Ok(Self::LessEqual),
      TT::RegexMatch => Ok(Self::RegexMatch),
      _ => Err(()),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
enum LogicOperator {
  And,
  Or,
}

impl TryFrom<&TokenType> for LogicOperator {
  type Error = ();

  fn try_from(value: &TokenType) -> Result<Self, Self::Error> {
    match value {
      TT::And => Ok(Self::And),
      TT::Or => Ok(Self::Or),
      _ => Err(()),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
  Value(Value),
  Simple(Simple),
  Compound(Compound),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compound {
  left: Box<Condition>,
  operator: LogicOperator,
  right: Box<Condition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Simple {
  left: Value,
  operator: ConditionalOperator,
  right: Value,
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Condition> {
  // loop {
  //   let left = parse_simple(ph)?;

  //   let left = if let Some(left) = left {
  //     left
  //   } else {
  //     return Condition::Value()
  //   }
  // }

  // let condition = if let Ok(operator) = LogicOperator::try_from(operator) {
  //   let compound = Compound {
  //     left: Box::new(value),
  //     operator,
  //     right: Box::new(parse(ph)?),
  //   };
  //   Condition::Compound(compound)
  // } else if let Ok(operator) = ConditionalOperator::try_from(operator) {
  // } else {
  //   Condition::Value(value)
  // };

  let stack = vec![];
  let rpn = vec![];

  while let Some(token) = ph.get(0) {
    let token = token.clone();

    let condition = if let Ok(operator) = LogicOperator::try_from(operator) {
      Condition::Compound(compound)
    } else if let Ok(operator) = ConditionalOperator::try_from(operator) {
    } else {
      Condition::Value(value)
    };

    ph.advance();
  }

  todo!();

  // Ok(condition)
}
