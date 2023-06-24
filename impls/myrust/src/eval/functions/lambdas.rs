use std::rc::Rc;

use crate::read::{AstNode, LambdaEntry};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(FnStarFn), Rc::new(IsFnFn)]
}

struct FnStarFn;
impl NativeFunction for FnStarFn {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "fn*".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, env) = data.destructure();

        let lambda_params = params.remove(0).try_unwrap_list_or_vector()?;
        let lambda_body = params.remove(0);

        let mut params_as_strings = vec![];
        for lambda_param in lambda_params.into_iter() {
            params_as_strings.push(lambda_param.try_unwrap_symbol()?);
        }

        let lambda = LambdaEntry {
            body: lambda_body,
            params: params_as_strings,
            is_macro: false,
            env: env.clone(),
        };

        return Ok(FunctionCallResultSuccess::Value(AstNode::Lambda(Rc::new(
            lambda,
        ))));
    }
}

struct IsFnFn;
impl NativeFunction for IsFnFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "fn?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        let node = data.destructure().0.remove(0);

        let is_lambda = match node {
            AstNode::Lambda(_) => true,
            AstNode::FunctionPtr(_) => true,
            _ => false,
        };
        return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(is_lambda)));
    }
}
