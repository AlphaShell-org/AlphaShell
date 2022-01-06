use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let this = self;
    write_f!(f, "{this.0}:{this.1}")
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Integer(i64),
  Float(f32),

  String(String),
  Identifier(String),

  // Keywords
  Export,
  Let,
  Import,
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
    let this = self;
    write_f!(f, "{this.r#type} at {this.position}")
  }
}

impl Default for TokenType {
  fn default() -> Self {
    Self::Invalid
  }
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
