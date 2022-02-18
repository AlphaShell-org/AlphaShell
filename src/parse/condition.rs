use super::value::Value;

#[derive(Debug, PartialEq, Clone)]
enum ConditonalOperator {
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le,
    Regex,
}

#[derive(Debug, PartialEq, Clone)]
enum LogicOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    Simple(Simple),
    Compound(Compound),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compound {
    left: Box<Condition>,
    operator: LogicOperator,
    right: Box<Condition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Simple {
    left: Value,
    operator: ConditonalOperator,
    right: Value,
}

pub fn parse(

