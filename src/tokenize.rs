mod error;

use error::{Error, Result};

use super::types::{Position, Token, TokenType, TT};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
  "let" => TT::Let,
  "export" => TT::Export,
  "import" => TT::Import,
  "source" => TT::Source,
  "fn" => TT::Function,
  "return" => TT::Return,
  "if" => TT::If,
  "elif" => TT::Elif,
  "else" => TT::Else,
  "for" => TT::For,
  "in" => TT::In,
  "while" => TT::While,
  "continue" => TT::Continue,
  "break" => TT::Break,
  "external" => TT::External,
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
            &format!("Encountered invalid character '{char}' while parsing a number"),
            state,
          ));
        }

        if let Ok(num) = buf.parse() {
          return Ok(Token::new(TT::Float(num), Position(state.line, start)));
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
        return Ok(Token::new(TT::Float(num), Position(state.line, start)));
      }
    }
    NumType::Int => {
      if let Ok(num) = buf.parse() {
        return Ok(Token::new(TT::Integer(num), Position(state.line, start)));
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

fn load_flag(state: &mut State) -> Result<Token> {
  let tmp_i = state.column;

  let mut dash_count = 0;
  let mut buf = String::new();

  while state.valid_char() && state.char() == '-' {
    buf.push(state.char());

    state.advance();
    dash_count += 1;
  }

  if dash_count > 2 {
    return Err(Error::new("Invalid flag format", state));
  }

  if state.valid_char() && (is_alpha(state.char()) || matches!(state.char(), '_' | '-' | '.' | '='))
  {
    buf.push(state.char());
    state.advance();
  }

  while state.valid_char() {
    let char = state.char();

    if !is_alpha_num(char) && !matches!(char, '_' | '-' | '.' | '=') {
      break;
    }

    buf.push(state.char());
    state.advance();
  }

  Ok(Token::new(TT::Flag(buf), Position(state.line, tmp_i)))
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
      &format!("No matching quote for '{quote_type}' found"),
      state,
    ));
  }

  // omit ending quote
  state.advance();

  let token_type = match quote_type {
    '"' => TT::String(buf),
    '\'' => TT::RawString(buf),
    _ => return Err(Error::new("Invalid string quote type", state)),
  };

  Ok(Token::new(token_type, Position(state.line, start)))
}

fn load_operator(state: &mut State) -> Result<Token> {
  let start = state.column;
  let char = state.char();

  macro_rules! operator_with_equal {
    ($a:ident, $b:ident) => {
      if state.next() == Some('=') {
        state.advance();

        TT::$b
      } else {
        TT::$a
      }
    };
  }

  let token_type = match char {
    '+' => operator_with_equal!(Add, AddAssignment),
    '-' => operator_with_equal!(Sub, SubAssignment),
    '*' => {
      if state.next() == Some('=') {
        state.advance();

        TT::MultiplyAssignment
      } else if state.next() == Some('*') {
        state.advance();

        if state.next() == Some('=') {
          state.advance();
          TT::PowerAssignment
        } else {
          TT::Power
        }
      } else {
        TT::Multiply
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
        TT::RegexMatch
      } else {
        return Err(Error::new("Unexpected token '~'", state));
      }
    }
    '.' => {
      if state.next() == Some('.') {
        state.advance();
        TT::Range
      } else {
        TT::Dot
      }
    }
    '|' => {
      if state.next() == Some('|') {
        state.advance();

        TT::Or
      } else {
        TT::Pipe
      }
    }
    '&' => {
      if state.next() == Some('&') {
        state.advance();

        TT::And
      } else {
        TT::Daemon
      }
    }
    '$' => TT::Dollar,
    '@' => TT::At,
    '?' => TT::Question,
    ',' => TT::Comma,
    ':' => TT::Colon,
    ';' => TT::Semicolon,
    '(' => TT::LParen,
    '[' => TT::LBracket,
    '{' => TT::LBrace,
    ')' => TT::RParen,
    ']' => TT::RBracket,
    '}' => TT::RBrace,
    _ => return Err(Error::new(&format!("Unknown character '{char}'"), state)),
  };

  state.advance();

  Ok(Token::new(token_type, Position(state.line, start)))
}

fn classify_str(str: &str) -> TokenType {
  if let Some(r#type) = KEYWORDS.get(str) {
    r#type.clone()
  } else {
    match str {
      "true" => TT::Boolean(true),
      "false" => TT::Boolean(false),
      _ => TT::Identifier(str.to_string()),
    }
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
      let tmp_i = state.column;

      let str = load_name(state);

      // check if string is a keyword
      let r#type = classify_str(&str);

      let token = Token::new(r#type, Position(state.line, tmp_i));

      state.tokens.push(token);

      continue;
    }

    if char == '$'
      && matches!(
        state.next(),
        Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
      )
    {
      let token = Token::new(
        TokenType::Identifier(state.next().unwrap().to_string()),
        Position(state.line, state.column),
      );

      state.tokens.push(token);
      state.advance_by(2);

      continue;
    }

    // flags
    if char == '-' {
      if let Some(char) = state.next() {
        if is_alpha(char) || char == '-' {
          let token = load_flag(state)?;
          state.tokens.push(token);
          continue;
        }
      }
    }

    // numbers
    if is_digit(char) {
      let token = load_number(state)?;
      state.tokens.push(token);
      continue;
    }

    // strings
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
