#[derive(Debug, Eq, PartialEq)]
pub enum BlockType {
  Foreach,
  Expression,
  Generic,
  Arithmetics,
  Identifier,
  FunctionCall,
  Condition,
  Raw,
}

#[derive(Debug)]
pub struct Transpiler {
  indent_char: &'static str,

  blocks: Vec<BlockType>,
}

impl Transpiler {
  pub fn new(indent_char: &'static str) -> Self {
    Self {
      indent_char,
      blocks: vec![],
    }
  }

  pub fn use_indent(&self, str: &str) -> String {
    let indent = self.indent_char.repeat(self.blocks.len());
    format!("{indent}{str}")
  }

  pub fn push_block(&mut self, block: BlockType) {
    self.blocks.push(block);
  }

  pub fn pop_block(&mut self) {
    debug_assert!(!self.blocks.is_empty(), "deindent at 0 indent_level");

    self.blocks.pop();
  }

  pub fn get_block(&self) -> Option<&BlockType> {
    self.blocks.last()
  }

  pub fn search(&self, block: &BlockType) -> bool {
    self.blocks.iter().any(|b| b == block)
  }
}
