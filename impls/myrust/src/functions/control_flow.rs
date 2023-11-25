use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(If), Rc::new(Do)]
}

struct If;
impl NativeFunction for If {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "if".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(3))?;

        let (mut params, env) = data.destructure();

        let condition = params.remove(0);
        let truebranch = params.remove(0);
        let falsebranch = if params.len() > 0 {
            params.remove(0)
        } else {
            AstNode::Nil
        };

        let condition_value = data.evaluator().eval(condition, env.clone())?;
        let branch_to_execute = match condition_value {
            AstNode::Bool(false) => falsebranch,
            AstNode::Nil => falsebranch,
            _ => truebranch,
        };

        Ok(FunctionCallResultSuccess::new_tailcall(
            branch_to_execute,
            env,
        ))
    }
}

struct Do;
impl NativeFunction for Do {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "do".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        let (mut params, env) = data.destructure();

        let evaluator = data.evaluator();
        while params.len() > 1 {
            let expression = params.remove(0);
            evaluator.eval(expression, env.clone())?;
        }

        // tail call
        Ok(FunctionCallResultSuccess::new_tailcall(
            params.remove(0),
            env,
        ))
    }
}
