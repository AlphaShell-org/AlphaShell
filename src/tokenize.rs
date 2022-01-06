mod error;

use super::types::{Position, Token, TokenType};
use error::{Error, Result};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
  "let" => TokenType::Let,
  "export" => TokenType::Export,
  "import" => TokenType::Import,
  "fn" => TokenType::Function,
  "return" => TokenType::Return,
  "if" => TokenType::If,
  "elif" => TokenType::Elif,
  "else" => TokenType::Else,
  "for" => TokenType::For,
  "in" => TokenType::In,
  "while" => TokenType::While,
  "continue" => TokenType::Continue,
  "break" => TokenType::Break,
};

pub struct State<'a> {
  lines: Vec<&'a str>,
  tokens: Vec<Token>,
  comment_block: usize,
  line: usize,
  column: usize,
}

impl<'a> State<'a> {
  pub fn new(lines: Vec<&'a str>) -> State<'a> {
    State {
      lines,
      tokens: Vec::new(),
      comment_block: 0,
      line: 0,
      column: 0,
    }
  }

  fn char_at(&self, line: usize, column: usize) -> char {
    self.lines[line].chars().nth(column).unwrap()
  }

  fn line(&self) -> &str {
    self.lines[self.line]
  }

  fn char(&self) -> char {
    self.char_at(self.line, self.column)
  }

  fn next(&self) -> Option<char> {
    if self.column + 1 < self.line().len() {
      Some(self.char_at(self.line, self.column + 1))
    } else {
      None
    }
  }

  fn prev(&self) -> char {
    self.char_at(self.line, self.column - 1)
  }

  fn valid_char(&self) -> bool {
    self.column < self.line().len()
  }

  fn advance(&mut self) {
    self.advance_by(1);
  }

  fn advance_by(&mut self, amount: usize) {
    self.column += amount;
  }

  fn advance_line(&mut self) {
    self.line += 1;
    self.column = 0;
  }

  fn to_pos(&self) -> Position {
    Position(self.line, self.column)
  }
}

fn is_whitespace(ch: char) -> bool {
  ch.is_ascii_whitespace()
}

fn is_digit(ch: char) -> bool {
  ch.is_ascii_digit()
}

fn is_alpha(ch: char) -> bool {
  ch.is_ascii_alphabetic()
}

fn is_alpha_num(ch: char) -> bool {
  ch.is_ascii_alphanumeric()
}

fn is_valid_num_char(ch: char) -> bool {
  is_digit(ch) || matches!(ch, '+' | '-' | '.')
}

fn load_number(state: &mut State) -> Result<Token> {
  enum NumType {
    Int,
    Float,
  }

  let mut buf = String::new();
  let start = state.column;
  let mut num_type = NumType::Int;
  let mut dot_encountered = false;

  while state.valid_char() && is_valid_num_char(state.char()) {
    let char = state.char();
    match char {
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
      '.' => {
        if state.next() == Some('.') {
          break;
        }

        if dot_encountered {
          return Err(Error::new(
            "Encountered second dot in number literal",
            state,
          ));
        }

        dot_encountered = true;

        if let Some(next) = state.next() {
          if is_digit(next) {
            num_type = NumType::Float;
          }
        } else {
          return Err(Error::new(
            "Expected digit after dot while parsing number",
            state,
          ));
        }
      }
      _ => {
        if is_alpha_num(char) {
          return Err(Error::new(
            &f!("Encountered invalid character '{char}' while parsing a number"),
            state,
          ));
        }

        if let Ok(num) = buf.parse() {
          return Ok(Token::new(
            TokenType::Float(num),
            Position(state.line, start),
          ));
        }
        return Err(Error::new("Invalid number", state));
      }
    }

    buf.push(char);
    state.advance();
  }

  match num_type {
    NumType::Float => {
      if let Ok(num) = buf.parse() {
        return Ok(Token::new(
          TokenType::Float(num),
          Position(state.line, start),
        ));
      }
    }
    NumType::Int => {
      if let Ok(num) = buf.parse() {
        return Ok(Token::new(
          TokenType::Integer(num),
          Position(state.line, start),
        ));
      }
    }
  };

  Err(Error::new("Invalid number", state))
}

fn load_name(state: &mut State) -> String {
  let mut buf = state.char().to_string();

  state.advance();

  while state.valid_char() {
    let char = state.char();
    if !is_alpha_num(char) && char != '_' {
      break;
    }
    buf.push(char);
    state.advance();
  }

  buf
}

fn load_string(state: &mut State) -> Result<Token> {
  let mut buf = String::new();

  let start = state.column;

  let quote_type = state.char();

  // omit beginning quote
  state.advance();

  while state.valid_char() && !(state.char() == quote_type && state.prev() != '\\') {
    buf.push(state.char());
    state.advance();
  }

  if state.char() != quote_type {
    return Err(Error::new(
      &f!("No matching quote for '{quote_type}' found"),
      state,
    ));
  }

  // omit ending quote
  state.advance();

  Ok(Token::new(
    TokenType::String(buf),
    Position(state.line, start),
  ))
}

fn load_operator(state: &mut State) -> Result<Token> {
  let start = state.column;
  let char = state.char();

  macro_rules! operator_with_equal {
    ($a:ident, $b:ident) => {
      if state.next() == Some('=') {
        state.advance();

        TokenType::$b
      } else {
        TokenType::$a
      }
    };
  }

  let token_type = match char {
    '+' => operator_with_equal!(Add, AddAssignment),
    '-' => operator_with_equal!(Sub, SubAssignment),
    '*' => {
      if state.next() == Some('=') {
        state.advance();

        TokenType::MultiplyAssignment
      } else if state.next() == Some('*') {
        state.advance();

        if state.next() == Some('=') {
          state.advance();
          TokenType::PowerAssignment
        } else {
          TokenType::Power
        }
      } else {
        TokenType::Multiply
      }
    }
    '/' => operator_with_equal!(Divide, DivideAssignment),
    '%' => operator_with_equal!(Modulo, ModuloAssignment),
    '=' => operator_with_equal!(Assignment, Equal),
    '<' => operator_with_equal!(Less, LessEqual),
    '>' => operator_with_equal!(Greater, GreaterEqual),
    '!' => operator_with_equal!(Not, NotEqual),
    '~' => {
      if state.next() == Some('=') {
        state.advance();
        TokenType::RegexMatch
      } else {
        return Err(Error::new("Unexpected token '~'", state));
      }
    }
    '.' => {
      if state.next() == Some('.') {
        state.advance();
        TokenType::Range
      } else {
        TokenType::Dot
      }
    }
    '|' => {
      if state.next() == Some('|') {
        state.advance();

        TokenType::Or
      } else {
        TokenType::Pipe
      }
    }
    '&' => {
      if state.next() == Some('&') {
        state.advance();

        TokenType::And
      } else {
        TokenType::Daemon
      }
    }
    '?' => TokenType::Question,
    ',' => TokenType::Comma,
    ':' => TokenType::Colon,
    ';' => TokenType::Semicolon,
    '(' => TokenType::LParen,
    '[' => TokenType::LBracket,
    '{' => TokenType::LBrace,
    ')' => TokenType::RParen,
    ']' => TokenType::RBracket,
    '}' => TokenType::RBrace,
    _ => return Err(Error::new(&f!("Unknown character '{char}'"), state)),
  };

  state.advance();

  Ok(Token::new(token_type, Position(state.line, start)))
}

fn classify_str(str: &str) -> TokenType {
  if let Some(r#type) = KEYWORDS.get(str) {
    r#type.clone()
  } else {
    TokenType::Identifier(str.to_string())
  }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>> {
  let lines: Vec<_> = s.lines().collect();
  let lines_length = lines.len();
  let mut state = State::new(lines);

  for _ in 0..lines_length {
    tokenize_line(&mut state)?;
    state.advance_line();
  }

  Ok(state.tokens)
}

fn tokenize_line(state: &mut State) -> Result<()> {
  while state.valid_char() {
    let char = state.char();

    if is_whitespace(char) {
      state.advance();
      continue;
    }

    if char == '*' && state.next() == Some('/') {
      if state.comment_block == 0 {
        return Err(Error::new(
          "encountered multi line comment terminator '*/' in non commented block",
          state,
        ));
      }

      state.advance_by(2);
      state.comment_block -= 1;
      continue;
    }

    if char == '/' && state.next() == Some('*') {
      state.advance_by(2);
      state.comment_block += 1;
      continue;
    }

    if state.comment_block != 0 {
      state.advance();
      continue;
    }

    if char == '/' && state.next() == Some('/') {
      break;
    };

    // names
    if is_alpha(char) || char == '_' {
      // for storing correct columns in tokens (+1 for zero indexing)
      let tmp_i = state.column;

      let str = load_name(state);

      // check if string is a keyword
      let r#type = classify_str(&str);

      state
        .tokens
        .push(Token::new(r#type, Position(state.line, tmp_i)));
      continue;
    }

    // numbers
    if is_digit(char) {
      let token = load_number(state)?;
      state.tokens.push(token);
      continue;
    }

    // let strings
    if state.char() == '"' || state.char() == '\'' {
      let token = load_string(state)?;
      state.tokens.push(token);
      continue;
    }

    // operators
    let operator = load_operator(state)?;
    state.tokens.push(operator);
  }

  Ok(())
}
