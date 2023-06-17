use std::rc::Rc;

use crate::eval::{EnvironmentEntry, EvalError, SharedEnvironment};

#[derive(Clone, PartialEq)]
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

#[allow(dead_code)]
impl AstNode {
    pub fn try_unwrap_int(self) -> Result<i64, EvalError> {
        match self {
            AstNode::Int(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Int".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_list(self) -> Result<Vec<AstNode>, EvalError> {
        match self {
            AstNode::List(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "List".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_string(self) -> Result<String, EvalError> {
        match self {
            AstNode::String(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "String".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_bool(self) -> Result<bool, EvalError> {
        match self {
            AstNode::Bool(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Bool".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_nil(self) -> Result<(), EvalError> {
        match self {
            AstNode::Nil => Ok(()),
            v => Err(EvalError::TypeError {
                expected: "Nil".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_symbol(self) -> Result<String, EvalError> {
        match self {
            AstNode::UnresolvedSymbol(name) => Ok(name),
            v => Err(EvalError::TypeError {
                expected: "Symbol".to_string(),
                got: v,
            }),
        }
    }
}
