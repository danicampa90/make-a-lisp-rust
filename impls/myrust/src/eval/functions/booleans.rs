use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(EqualOp), Rc::new(LessThanOp), Rc::new(NandOp)]
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

        let (mut params, env) = data.destructure();

        let a = params.remove(0);
        let b = params.remove(0);

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(a == b)));
    }
}

struct LessThanOp;
impl NativeFunction for LessThanOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "<".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, env) = data.destructure();

        let a = params.remove(0).try_unwrap_int()?;
        let b = params.remove(0).try_unwrap_int()?;

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(a < b)));
    }
}

struct NandOp;
impl NativeFunction for NandOp {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "nand".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, env) = data.destructure();

        let a = params.remove(0).try_unwrap_bool()?;
        let b = params.remove(0).try_unwrap_bool()?;

        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(!(a && b))));
    }
}

/*
// Step 4: Bool operators & boolean logic
env.set_owned(EnvironmentEntry::new_native_function(
    "=".to_string(),
    |params, _env| {
        check_param_count(&params, 2, "=")?;
        Ok(AstNode::Bool(params[0] == params[1]))
    },
));

env.set_owned(EnvironmentEntry::new_native_function(
    "<".to_string(),
    |params, _env| {
        check_param_count(&params, 2, "<")?;
        match (&params[0], &params[1]) {
            (AstNode::Int(a), AstNode::Int(b)) => Ok(AstNode::Bool(a < b)),
            _ => Err(EvalError::custom_exception_str(
                "< has some invalid parameter types",
            )),
        }
    },
));
env.set_owned(EnvironmentEntry::new_native_function(
    "nand".to_string(),
    |params, _env| {
        check_param_count(&params, 2, "nand")?;
        match (&params[0], &params[1]) {
            (AstNode::Bool(a), AstNode::Bool(b)) => Ok(AstNode::Bool(!(*a && *b))),
            _ => Err(EvalError::custom_exception_str(
                "or has some invalid parameter types",
            )),
        }
    },
));*/
