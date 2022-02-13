#[derive(Debug, Eq, PartialEq)]
pub enum BlockType {
  Foreach,
  Expression,
  Import,
}

pub struct Transpiler {
  indent_level: usize,
  indent_char: &'static str,

  block_type: Option<BlockType>,
}

impl Transpiler {
  pub fn new(
    indent_level: usize,
    indent_char: &'static str,
    block_type: Option<BlockType>,
  ) -> Self {
    Self {
      indent_level,
      indent_char,
      block_type,
    }
  }

  pub fn use_indent(&self, str: &str) -> String {
    let indent = self.indent_char.repeat(self.indent_level);
    format!("{indent}{str}")
  }

  pub fn indent(&mut self) {
    self.indent_level += 1;
  }

  pub fn deindent(&mut self) {
    assert!(self.indent_level != 0, "deindent at 0 indent_level");

    self.indent_level -= 1;
  }

  pub fn get_block(&self) -> &Option<BlockType> {
    &self.block_type
  }

  pub fn change_block(&mut self, block_type: BlockType) {
    self.block_type = Some(block_type);
  }

  pub fn reset_block(&mut self) {
    self.block_type = None;
  }
}
