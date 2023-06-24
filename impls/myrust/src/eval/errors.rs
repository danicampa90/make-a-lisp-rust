use std::fmt::Display;

use crate::read::AstNode;

#[derive(Debug)]
pub enum EvalError {
    SymbolNotFound(String),
    InvalidFunctionCallNodeType(AstNode),
    ParameterCountError {
        expected_min: Option<usize>,
        expected_max: Option<usize>,
        provided: usize,
    },
    TypeError {
        expected: String,
        got: AstNode,
    },
    CustomException(AstNode),
}

impl EvalError {
    pub fn custom_exception_str<T>(s: T) -> EvalError
    where
        T: ToString,
    {
        EvalError::CustomException(AstNode::String(s.to_string()))
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::SymbolNotFound(symbol) => write!(f, "\'{}\' not found", symbol),
            EvalError::InvalidFunctionCallNodeType(node) => {
                write!(f, "Cannot call \'{}\' - expecting a function pointer", node)
            }
            EvalError::ParameterCountError {
                expected_min,
                expected_max,
                provided,
            } => write!(
                f,
                "Invalid number of parameters: minimum: {:?}, maximum:{:?}, provided:{}",
                expected_min, expected_max, provided
            ),
            EvalError::TypeError { expected, got } => {
                write!(f, "Expected \'{}\' - found \'{}\'", expected, got)
            }
            EvalError::CustomException(node) => write!(f, "Custom exception: {:?}", node),
        }
    }
}
