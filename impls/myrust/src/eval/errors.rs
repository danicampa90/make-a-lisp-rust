use crate::read::AstNode;

#[derive(Debug)]
pub enum EvalError {
    SymbolNotFound(String),
    InvalidFunctionCallNodeType(AstNode),
    NotACallableFunction(String),
    CustomException(AstNode),
}

impl EvalError {
    pub fn custom_exception_string(s: &str) -> EvalError {
        EvalError::CustomException(AstNode::String(s.to_string()))
    }
}
