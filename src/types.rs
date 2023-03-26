use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Position(line, column) = self;
    write!(f, "{line}:{column}")
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub r#type: TokenType,
  pub position: Position,
}

impl Token {
  pub fn new(r#type: TokenType, position: Position) -> Self {
    Self { r#type, position }
  }
}

pub type TT = TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Integer(i32),
  Float(f32),
  Boolean(bool),

  String(String),
  RawString(String),
  Identifier(String),

  Flag(String),

  // Keywords
  Export,
  Let,
  External,
  Import,
  Source,
  Function,
  Return,
  If,
  Elif,
  Else,
  For,
  In,
  While,
  Continue,
  Break,
  Switch,
  Case,

  // Operators
  Assignment,

  // Arithmetic
  Add,
  Sub,
  Multiply,
  Divide,
  Modulo,
  Power, // **

  // UnaryAdd,
  // UnarySub,
  AddAssignment,
  SubAssignment,
  MultiplyAssignment,
  DivideAssignment,
  ModuloAssignment,
  PowerAssignment, // **

  // Logic
  And,
  Or,
  Not,
  Equal,
  Less,
  Greater,
  LessEqual,
  GreaterEqual,
  NotEqual,

  // IO
  Pipe,

  // daemon
  Daemon,

  // regex
  RegexMatch,

  // Conditional ( ? : )
  Question,
  Colon, // :

  Dollar,
  At,

  // Separators
  Dot,
  Range, //..
  Comma,
  Semicolon,

  // Parenthesis, Braces, Brackets
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,

  Invalid,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Token { r#type, .. } = self;
    write!(f, "{type}")
  }
}

impl Default for TokenType {
  fn default() -> Self {
    Self::Invalid
  }
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::Integer(x) => write!(f, "{x}"),
      TokenType::Float(x) => write!(f, "{x}"),
      TokenType::Boolean(x) => write!(f, "{x}"),
      TokenType::String(x)
      | TokenType::RawString(x)
      | TokenType::Identifier(x)
      | TokenType::Flag(x) => write!(f, "{x}"),

      _ => write!(f, "{self:?}"),
    }
  }
}
