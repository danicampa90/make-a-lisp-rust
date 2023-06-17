use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(MathBinaryOp::Plus),
        Rc::new(MathBinaryOp::Minus),
        Rc::new(MathBinaryOp::Times),
        Rc::new(MathBinaryOp::Divide),
    ]
}

enum MathBinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
}

impl NativeFunction for MathBinaryOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Times => "*",
            Self::Divide => "/",
        }
        .to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;
        let (mut ast, env) = data.destructure();
        let a = ast.remove(0).try_unwrap_int()?;
        let b = ast.remove(0).try_unwrap_int()?;
        let result = match self {
            Self::Plus => a + b,
            Self::Minus => a - b,
            Self::Times => a * b,
            Self::Divide => a / b,
        };

        Ok(FunctionCallResultSuccess::Value(AstNode::Int(result)))
    }
}
