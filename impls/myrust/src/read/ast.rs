use std::{clone, fmt::Display, rc::Rc};

use crate::eval::EnvironmentEntry;

#[derive(Clone)]
pub enum AstNode {
    List(Vec<AstNode>),
    String(String),
    Int(i64),
    FunctionPtr(Rc<EnvironmentEntry>), // internal only: a function pointer, like a lambda. saved in a variable
    UnresolvedSymbol(String), // only existing during parsing. Unresolved symbols get resolved into a function pointer during evaluation.
}
