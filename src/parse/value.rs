use super::{
  array,
  error::{Error, ParserResult},
  function_call::{self, FunctionCall},
  map,
  node::Node,
  parse_helper::ParseHelper,
};
use crate::types::{TokenType, TT};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
  Add,
  Sub,
  Multiply,
  Divide,
  And,
  Or,
}

impl From<&TokenType> for BinaryOperator {
  fn from(token: &TokenType) -> Self {
    match token {
      TT::Add => Self::Add,
      TT::Sub => Self::Sub,
      TT::Multiply => Self::Multiply,
      TT::Divide => Self::Divide,
      TT::And => Self::And,
      TT::Or => Self::Or,
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
  Identifier(String),
  String(String),
  Int(i32),
  Float(f32),
  Array(Vec<String>),
  Map(Vec<(String, String)>),
  FunctionCall(FunctionCall),
}

impl From<&TokenType> for Data {
  fn from(token: &TokenType) -> Self {
    match token {
      TT::Integer(num) => Self::Int(*num),
      TT::Float(num) => Self::Float(*num),
      TT::String(string) => Self::String(string.clone()),
      TT::Identifier(name) => Self::Identifier(name.clone()),
      TT::At => Self::Identifier("@".into()),
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
  pub left: Box<Value>,
  pub operator: BinaryOperator,
  pub right: Box<Value>,
}

impl Expression {
  pub fn new(left: Box<Value>, operator: BinaryOperator, right: Box<Value>) -> Self {
    Self {
      left,
      operator,
      right,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Expression(Expression),
  Raw(Data),
}

fn parse_value(ph: &mut ParseHelper) -> ParserResult<Value> {
  let token = ph.peek(0);

  let value = match token {
    Some(TokenType::Identifier(..)) if ph.peek(1) == Some(&TT::LParen) => {
      let call = function_call::parse_inner(ph)?;
      return Ok(Value::Raw(Data::FunctionCall(call)));
    }
    Some(TT::Identifier(..) | TT::String(..) | TT::Integer(..) | TT::Float(..) | TT::At) => {
      Ok(Value::Raw(token.unwrap().into()))
    }
    Some(TT::LBracket) => Ok(array::parse(ph)?),
    Some(TT::LBrace) => Ok(map::parse(ph)?),
    Some(_) => Err(Error::unexpected(ph)),
    None => Err(Error::end(ph)),
  };

  ph.advance();

  value
}

pub fn parse(ph: &mut ParseHelper) -> ParserResult<Node> {
  let left = parse_value(ph)?;

  let token = ph.peek(0);
  let operator = match token {
    Some(TT::Add | TT::Sub | TT::Multiply | TT::Modulo) => token.unwrap().into(),
    Some(_) => return Ok(Node::Value(left)),
    None => return Err(Error::end(ph)),
  };

  ph.advance();

  let right = parse_value(ph)?;

  Ok(Node::Value(Value::Expression(Expression::new(
    Box::new(left),
    operator,
    Box::new(right),
  ))))
}
