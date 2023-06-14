use crate::read::AstNode;

#[derive(Debug)]
pub enum EvalError {
    SymbolNotFound(String),
    InvalidFunctionCallNodeType(AstNode),
    NotACallableFunction(String),
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
