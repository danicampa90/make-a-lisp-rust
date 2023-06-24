use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(EqualOp),
        Rc::new(NumComparisonOp::Gt),
        Rc::new(NumComparisonOp::Lt),
        Rc::new(NumComparisonOp::Ge),
        Rc::new(NumComparisonOp::Le),
        Rc::new(BooleanBinaryOp::And),
        Rc::new(BooleanBinaryOp::Or),
        Rc::new(BooleanBinaryOp::Nand),
        Rc::new(BooleanBinaryOp::Nor),
        Rc::new(BooleanBinaryOp::Xor),
        Rc::new(NotOp),
    ]
}

struct EqualOp;
impl NativeFunction for EqualOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "=".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, _env) = data.destructure();

        let a = params.remove(0);
        let b = params.remove(0);

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(a == b)));
    }
}

enum NumComparisonOp {
    Gt,
    Lt,
    Le,
    Ge,
}
impl NativeFunction for NumComparisonOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        match self {
            NumComparisonOp::Gt => ">",
            NumComparisonOp::Lt => "<",
            NumComparisonOp::Le => "<=",
            NumComparisonOp::Ge => ">=",
        }
        .to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, _env) = data.destructure();

        let a = params.remove(0).try_unwrap_int()?;
        let b = params.remove(0).try_unwrap_int()?;
        let result = match self {
            NumComparisonOp::Gt => a > b,
            NumComparisonOp::Lt => a < b,
            NumComparisonOp::Le => a <= b,
            NumComparisonOp::Ge => a >= b,
        };

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(result)));
    }
}

enum BooleanBinaryOp {
    And,
    Or,
    Nand,
    Nor,
    Xor,
}
impl NativeFunction for BooleanBinaryOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        match self {
            BooleanBinaryOp::And => "and",
            BooleanBinaryOp::Or => "or",
            BooleanBinaryOp::Nand => "nand",
            BooleanBinaryOp::Nor => "nor",
            BooleanBinaryOp::Xor => "xor",
        }
        .to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, _env) = data.destructure();

        let a = params.remove(0).try_unwrap_bool()?;
        let b = params.remove(0).try_unwrap_bool()?;

        let result = match self {
            BooleanBinaryOp::And => a && b,
            BooleanBinaryOp::Or => a || b,
            BooleanBinaryOp::Nand => !(a && b),
            BooleanBinaryOp::Nor => !(a || b),
            BooleanBinaryOp::Xor => (!a && b) || (a && !b),
        };

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(result)));
    }
}

struct NotOp;
impl NativeFunction for NotOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "not".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        let (mut params, _env) = data.destructure();

        let a = match params.remove(0) {
            AstNode::Bool(val) => val,
            AstNode::Nil => false,
            _ => true,
        };

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(!a)));
    }
}
