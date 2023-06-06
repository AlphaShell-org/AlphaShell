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

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Int(i32),
  Float(f32),
  Bool(bool),
  String(String),
  RawString(String),
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
  Power,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  RegexMatch,
  And,
  Or,
}

impl BinaryOperator {
  pub fn try_from_token(token: &TokenType) -> Option<Self> {
    match token {
      TT::Add => Some(Self::Add),
      TT::Sub => Some(Self::Sub),
      TT::Multiply => Some(Self::Multiply),
      TT::Divide => Some(Self::Divide),
      TT::Modulo => Some(Self::Modulo),
      TT::Power => Some(Self::Power),
      TT::Equal => Some(Self::Equal),
      TT::NotEqual => Some(Self::NotEqual),
      TT::Greater => Some(Self::Greater),
      TT::GreaterEqual => Some(Self::GreaterEqual),
      TT::Less => Some(Self::Less),
      TT::LessEqual => Some(Self::LessEqual),
      TT::RegexMatch => Some(Self::RegexMatch),
      TT::And => Some(Self::And),
      TT::Or => Some(Self::Or),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssignmentOperator {
  Assignment,
  AddAssignment,
  SubAssignment,
  MultiplyAssignment,
  DivideAssignment,
  ModuloAssignment,
  PowerAssignment,
}
impl AssignmentOperator {
  pub fn try_from_token(token: &TokenType) -> Option<Self> {
    match token {
      TT::Assignment => Some(Self::Assignment),
      TT::AddAssignment => Some(Self::AddAssignment),
      TT::SubAssignment => Some(Self::SubAssignment),
      TT::MultiplyAssignment => Some(Self::MultiplyAssignment),
      TT::DivideAssignment => Some(Self::DivideAssignment),
      TT::ModuloAssignment => Some(Self::ModuloAssignment),
      TT::PowerAssignment => Some(Self::PowerAssignment),

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
  Assignment(Box<Value>, AssignmentOperator, Box<Value>),
  FunctionCall(FunctionCall),
  Parenthesized(Box<Value>),
}

fn parse_single(ph: &mut ParseHelper) -> ParserResult<Value> {
  let token = ph.peek(0);

  let mut value = match token {
    Some(TT::LParen) => {
      ph.advance();
      let value = parse_inner(ph)?;
      check_token!(ph, TT::RParen);
      ph.advance();
      Ok(Value::Parenthesized(Box::new(value)))
    }

    Some(TT::Identifier(..) | TT::Dollar) if ph.peek(1) == Some(&TT::LParen) => {
      Ok(Value::FunctionCall(function_call::parse_inner(ph)?))
    }

    Some(TT::Identifier(name)) => {
      if ph.variables.get(name).is_none() {
        return Err(Error::undefined_variable(ph));
      }

      if ph.peek(1) == Some(&TT::Pipe) {
        return Ok(Value::FunctionCall(function_call::parse_inner(ph)?));
      }

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

    Some(TT::RawString(string)) => {
      if ph.peek(1) == Some(&TT::Pipe) {
        return Ok(Value::FunctionCall(function_call::parse_inner(ph)?));
      }

      let string = string.clone();
      ph.advance();
      Ok(Value::Literal(Literal::RawString(string)))
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

    Some(TT::Boolean(boolean)) => {
      let boolean = *boolean;
      ph.advance();
      Ok(Value::Literal(Literal::Bool(boolean)))
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

  while let Ok(val) = &value {
    if ph.peek(0) != Some(&TT::LBracket) {
      break;
    }

    ph.advance();

    let member = parse_single(ph)?;

    value = Ok(Value::MemberExpression(
      Box::new(val.clone()),
      Box::new(member),
    ));

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
    } else if let Some(operator) = AssignmentOperator::try_from_token(token) {
      match &left {
        Value::Identifier(_) | Value::MemberExpression(_, _) => {}
        _ => return Err(Error::new("Can't assign to this expression", ph.get(-1))),
      }

      ph.advance();
      let right = parse_single(ph)?;
      left = Value::Assignment(Box::new(left), operator, Box::new(right));
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

pub fn parse_inline_let(ph: &mut ParseHelper) -> ParserResult<(String, FunctionCall)> {
  check_token!(ph, TT::Let);
  ph.advance();

  let name = match ph.peek(0) {
    Some(TT::Identifier(name)) => name.clone(),
    Some(_) => return Err(Error::unexpected(ph)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  check_token!(ph, TT::Assignment);

  ph.advance();

  let call = function_call::parse_inner(ph)?;

  Ok((name, call))
}
