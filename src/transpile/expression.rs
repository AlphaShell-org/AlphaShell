use super::{
  error::{Error, TranspileResult},
  transpiler::Transpiler,
  value,
};
use crate::parse::node::Node;

// pub fn transpile_inner(
//   t: &mut Transpiler,
//   assignment: &Assignment,
//   node: &Node,
// ) -> TranspileResult<String> {
//   let Assignment { lhs, operator, rhs } = assignment;

//   let rhs = value::transpile_inner(t, rhs, node)?;

//   t.indent(BlockType::Identifier);
//   let lhs = value::transpile_inner(t, lhs, node)?;
//   t.deindent();

//   let operator = match operator {
//     AssignementType::Assignment => "=",
//     AssignementType::AddAssignment => "+=",
//     AssignementType::SubAssignment => "-=",
//     AssignementType::MultiplyAssignment => "*=",
//     AssignementType::DivideAssignment => "/=",
//     AssignementType::ModuloAssignment => "%=",
//     AssignementType::PowerAssignment => "**=",
//   };

//   let string = format!("{lhs}{operator}{rhs}");

//   Ok(t.use_indent(&string))
// }

pub fn transpile(t: &mut Transpiler, node: &Node) -> TranspileResult<String> {
  match node {
    Node::Expression(value) => value::transpile_inner(t, value, node),
    _ => Err(Error::new("Invalid node type", node)),
  }
}
