use super::{
  array,
  error::{Error, ParserResult},
  function_call::{self, FunctionCall},
  map,
  node::Node,
  parse_helper::ParseHelper,
};
use crate::{
  check_token,
  types::{TokenType, TT},
};

// #[derive(Debug, PartialEq, Clone)]
// enum Operation {
//   Add(Value, Value),
//   Sub(Value, Value),
//   Multiply(Value, Value),
//   Divide(Value, Value),
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum Value {
//   Operation(Box<Operation>),
//   Identifier(String),
//   String(String),
//   Int(i32),
//   Float(f32),
//   Array(Vec<String>),
//   Map(Vec<(String, String)>),
//   FunctionCall(FunctionCall),
// }

// impl From<&TokenType> for Value {
//   fn from(token: &TokenType) -> Self {
//     match token {
//       TT::Integer(num) => Self::Int(*num),
//       TT::Float(num) => Self::Float(*num),
//       TT::String(string) => Self::String(string.clone()),
//       TT::Identifier(name) => Self::Identifier(name.clone()),
//       TT::At => Self::Identifier("@".into()),
//       _ => unimplemented!(),
//     }
//   }
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct Expression {
//   pub left: Box<Value>,
//   pub operator: BinaryOperator,
//   pub right: Box<Value>,
// }

// impl Expression {
//   pub fn new(left: Box<Value>, operator: BinaryOperator, right: Box<Value>) -> Self {
//     Self {
//       left,
//       operator,
//       right,
//     }
//   }
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum Value {
//   Expression(Expression),
//   Raw(Value),
// }

// fn parse_value(ph: &mut ParseHelper) -> ParserResult<Value> {
//   let token = ph.peek(0);

//   let value = match token {
//     Some(TokenType::Identifier(..)) if ph.peek(1) == Some(&TT::LParen) => {
//       let call = function_call::parse_inner(ph)?;
//       return Ok(Value::Raw(Value::FunctionCall(call)));
//     }
//     Some(TT::Identifier(..) | TT::String(..) | TT::Integer(..) | TT::Float(..) | TT::At) => {
//       Ok(Value::Raw(token.unwrap().into()))
//     }
//     Some(TT::LBracket) => Ok(array::parse(ph)?),
//     Some(TT::LBrace) => Ok(map::parse(ph)?),
//     Some(_) => Err(Error::unexpected(ph)),
//     None => Err(Error::end(ph)),
//   };

//   ph.advance();

//   value
// }

// pub fn parse_inner(ph: &mut ParseHelper) -> ParserResult<Value> {
//   let left = parse_value(ph)?;

//   let token = ph.peek(0);
//   let operator = match token {
//     Some(TT::Add | TT::Sub | TT::Multiply | TT::Modulo) => token.unwrap().into(),
//     Some(_) => return Ok(left),
//     None => return Err(Error::end(ph)),
//   };

//   ph.advance();

//   let right = parse_value(ph)?;

//   Ok(Value::Expression(Expression::new(
//     Box::new(left),
//     operator,
//     Box::new(right),
//   )))
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Int(i32),
  Float(f32),
  String(String),
  Array(Vec<Value>),
  Map(Vec<(String, Value)>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryOperator {
  Not,
  Minus,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOperator {
  Add,
  Sub,
  Multiply,
  Divide,
  Modulo,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  RegexMatch,
  And,
  Or,

  Assignment,
  AddAssignment,
  SubAssignment,
  MultiplyAssignment,
  DivideAssignment,
  ModuloAssignment,
  PowerAssignment,
}

impl BinaryOperator {
  pub fn try_from_token(token: &TokenType) -> Option<Self> {
    match token {
      TT::Add => Some(BinaryOperator::Add),
      TT::Sub => Some(BinaryOperator::Sub),
      TT::Multiply => Some(BinaryOperator::Multiply),
      TT::Divide => Some(BinaryOperator::Divide),
      TT::Modulo => Some(BinaryOperator::Modulo),
      TT::Equal => Some(BinaryOperator::Equal),
      TT::NotEqual => Some(BinaryOperator::NotEqual),
      TT::Greater => Some(BinaryOperator::Greater),
      TT::GreaterEqual => Some(BinaryOperator::GreaterEqual),
      TT::Less => Some(BinaryOperator::Less),
      TT::LessEqual => Some(BinaryOperator::LessEqual),
      TT::RegexMatch => Some(BinaryOperator::RegexMatch),
      TT::And => Some(BinaryOperator::And),
      TT::Or => Some(BinaryOperator::Or),
      TT::Assignment => Some(BinaryOperator::Assignment),
      TT::AddAssignment => Some(BinaryOperator::AddAssignment),
      TT::SubAssignment => Some(BinaryOperator::SubAssignment),
      TT::MultiplyAssignment => Some(BinaryOperator::MultiplyAssignment),
      TT::DivideAssignment => Some(BinaryOperator::DivideAssignment),
      TT::ModuloAssignment => Some(BinaryOperator::ModuloAssignment),
      TT::PowerAssignment => Some(BinaryOperator::PowerAssignment),

      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Literal(Literal),
  Identifier(String),
  UnaryExpression(UnaryOperator, Box<Value>),
  BinaryExpression(Box<Value>, BinaryOperator, Box<Value>),
  TernaryExpression(Box<Value>, Box<Value>, Box<Value>),
  MemberExpression(Box<Value>, Box<Value>),
  FunctionCall(FunctionCall),
}

fn parse_single(ph: &mut ParseHelper) -> ParserResult<Value> {
  let token = ph.peek(0);
  let value = match token {
    Some(TT::Identifier(..) | TT::Dollar) if ph.peek(1) == Some(&TT::LParen) => {
      Ok(Value::FunctionCall(function_call::parse_inner(ph)?))
    }

    Some(TT::Identifier(name)) => {
      let name = name.clone();
      ph.advance();
      Ok(Value::Identifier(name))
    }

    Some(TT::At) => {
      ph.advance();
      Ok(Value::Identifier("@".into()))
    }

    Some(TT::String(string)) => {
      if ph.peek(1) == Some(&TT::Pipe) {
        return Ok(Value::FunctionCall(function_call::parse_inner(ph)?));
      }

      let string = string.clone();
      ph.advance();
      Ok(Value::Literal(Literal::String(string)))
    }
    Some(TT::Integer(num)) => {
      let num = *num;
      ph.advance();
      Ok(Value::Literal(Literal::Int(num)))
    }
    Some(TT::Float(num)) => {
      let num = *num;
      ph.advance();
      Ok(Value::Literal(Literal::Float(num)))
    }

    Some(TT::LBracket) => Ok(Value::Literal(Literal::Array(array::parse(ph)?))),
    Some(TT::LBrace) => Ok(Value::Literal(Literal::Map(map::parse(ph)?))),

    Some(TT::Not) => {
      ph.advance();
      let value = parse_single(ph)?;
      Ok(Value::UnaryExpression(UnaryOperator::Not, Box::new(value)))
    }

    Some(TT::Sub) => {
      ph.advance();
      let value = parse_single(ph)?;
      Ok(Value::UnaryExpression(
        UnaryOperator::Minus,
        Box::new(value),
      ))
    }

    Some(_) => Err(Error::unexpected(ph)),
    None => Err(Error::end(ph)),
  };

  let mut value = value;

  while let Ok(val) = &value && ph.peek(0) == Some(&TT::LBracket) {
    ph.advance();

    let member = parse_single(ph)?;

    value = Ok(Value::MemberExpression(Box::new(val.clone()), Box::new(member)));

    check_token!(ph, TT::RBracket);

    ph.advance();
  }

  value
}

fn parse_stage_1(ph: &mut ParseHelper) -> ParserResult<Value> {
  let mut left = parse_single(ph)?;

  while let Some(token) = ph.peek(0) {
    if let Some(operator) = BinaryOperator::try_from_token(token) {
      ph.advance();
      let right = parse_single(ph)?;
      left = Value::BinaryExpression(Box::new(left), operator, Box::new(right));
    } else {
      return Ok(left);
    }
  }

  Ok(left)
}

pub fn parse_inner(ph: &mut ParseHelper) -> ParserResult<Value> {
  let mut value = parse_stage_1(ph)?;

  if let Some(TT::Question) = ph.peek(0) {
    ph.advance();

    let true_value = parse_stage_1(ph)?;

    check_token!(ph, TT::Colon);
    ph.advance();

    let false_value = parse_stage_1(ph)?;

    value = Value::TernaryExpression(Box::new(value), Box::new(true_value), Box::new(false_value));
  }

  Ok(value)
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  Ok(Node::Value(parse_inner(ph)?))
}
