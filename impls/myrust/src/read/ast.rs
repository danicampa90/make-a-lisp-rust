use std::rc::Rc;

use crate::eval::{EnvironmentEntry, SharedEnvironment};

#[derive(Clone)]
pub enum AstNode {
    List(Vec<AstNode>),
    String(String),
    Int(i64),
    Bool(bool),
    Nil,
    FunctionPtr(Rc<EnvironmentEntry>), // internal only: a function pointer, like a lambda. saved in a variable
    Lambda(
        Rc<(
            Vec<String>,       /* params */
            AstNode,           /* body */
            SharedEnvironment, /* Environment */
        )>,
    ),
    UnresolvedSymbol(String), // only existing during parsing. Unresolved symbols get resolved into a function pointer during evaluation.
}
