use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::eval::{EnvironmentEntry, EvalError, SharedEnvironment};

#[derive(Clone, PartialEq)]
pub enum AstNode {
    List(Vec<AstNode>),
    Vector(Vec<AstNode>),
    HashMap(HashMap<String, AstNode>),
    Atom(Rc<RefCell<AstNode>>),
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

    pub fn try_unwrap_list_or_vector(self) -> Result<Vec<AstNode>, EvalError> {
        match self {
            AstNode::List(i) => Ok(i),
            AstNode::Vector(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "List or Vector".to_string(),
                got: v,
            }),
        }
    }

    pub fn try_unwrap_vector(self) -> Result<Vec<AstNode>, EvalError> {
        match self {
            AstNode::Vector(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Vector".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_hashmap(self) -> Result<HashMap<String, AstNode>, EvalError> {
        match self {
            AstNode::HashMap(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Vector".to_string(),
                got: v,
            }),
        }
    }
    pub fn try_unwrap_atom(self) -> Result<Rc<RefCell<AstNode>>, EvalError> {
        match self {
            AstNode::Atom(i) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Atom".to_string(),
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
