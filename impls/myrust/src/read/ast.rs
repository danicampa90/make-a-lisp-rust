use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::eval::{EnvironmentEntry, EvalError, SharedEnvironment};

use super::Lexer;

#[derive(Clone)]
pub struct LambdaEntry {
    pub params: Vec<String>,
    pub body: AstNode,
    pub env: SharedEnvironment,
    pub is_macro: bool,
}

impl LambdaEntry {
    pub fn set_is_macro(mut self, is_macro: bool) -> Self {
        self.is_macro = is_macro;
        self
    }
}

impl PartialEq for LambdaEntry {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.params == other.params
            && self.env == other.env
            && self.is_macro == other.is_macro
    }
}

#[derive(Clone)]
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
    Lambda(Rc<LambdaEntry>),
    UnresolvedSymbol(String), // only existing during parsing. Unresolved symbols get resolved into a function pointer during evaluation.
}

impl PartialEq for AstNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Vector(l0), Self::Vector(r0)) => l0 == r0,
            (Self::Vector(l0), Self::List(r0)) => l0 == r0, // < list and vectors should be comparing equal
            (Self::List(l0), Self::Vector(r0)) => l0 == r0, // < list and vectors should be comparing equal
            (Self::HashMap(l0), Self::HashMap(r0)) => l0 == r0,
            (Self::Atom(l0), Self::Atom(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::FunctionPtr(l0), Self::FunctionPtr(r0)) => l0 == r0,
            (Self::Lambda(l0), Self::Lambda(r0)) => l0 == r0,
            (Self::UnresolvedSymbol(l0), Self::UnresolvedSymbol(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
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
                expected: "Hashmap".to_string(),
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
    pub fn try_unwrap_keyword(self) -> Result<String, EvalError> {
        match self {
            AstNode::String(i) if i.starts_with(Lexer::KEYWORD_PREFIX) => Ok(i),
            v => Err(EvalError::TypeError {
                expected: "Keyword".to_string(),
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

    pub fn create_keyword(name: &str) -> AstNode {
        return AstNode::String(Lexer::KEYWORD_PREFIX.to_string() + name);
    }
}
