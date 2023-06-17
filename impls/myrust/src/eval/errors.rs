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
    CustomException(String),
}

impl EvalError {
    pub fn custom_exception_str<T>(s: T) -> EvalError
    where
        T: ToString,
    {
        EvalError::CustomException(s.to_string())
    }
}
