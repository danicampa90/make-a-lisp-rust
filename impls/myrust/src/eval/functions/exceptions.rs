use std::rc::Rc;

use crate::{
    eval::{Environment, EnvironmentEntry, EvalError},
    read::AstNode,
};

use super::{
    FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction, TailCallData,
};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(TryStarFn), Rc::new(ThrowFn)]
}

struct TryStarFn;
impl NativeFunction for TryStarFn {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "try*".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;
        let evaluator = data.evaluator();
        let (mut ast, env) = data.destructure();

        let to_run = ast.remove(0);
        let mut catch_clause = ast.remove(0).try_unwrap_list_or_vector()?;

        let result = evaluator.eval(to_run, env.clone());

        if let Err(x) = result {
            if catch_clause.remove(0).try_unwrap_symbol()? != "catch*" {
                return Err(EvalError::custom_exception_str(
                    "The second parameter of try* should be starting with (catch*)",
                ));
            }
            let exception_data = match x {
                EvalError::SymbolNotFound(symbol) => {
                    AstNode::String(format!("\'{}\' not found", symbol))
                }
                EvalError::InvalidFunctionCallNodeType(node) => AstNode::List(vec![
                    AstNode::create_keyword("InvalidFunctionCallNodeType"),
                    node,
                ]),
                EvalError::ParameterCountError {
                    expected_min: _,
                    expected_max: _,
                    provided: _,
                } => AstNode::List(vec![
                    AstNode::create_keyword("ParameterCount"), /* it would be great to provide more parameters, but whatever */
                ]),
                EvalError::TypeError { expected, got } => AstNode::List(vec![
                    AstNode::create_keyword("TypeError"),
                    AstNode::String(expected),
                    got,
                ]),
                EvalError::CustomException(custom) => custom,
            };

            let name = catch_clause.remove(0).try_unwrap_symbol()?;
            let handler = catch_clause.remove(0);
            let mut handler_env = Environment::new_child(env);
            handler_env.set(Rc::new(EnvironmentEntry::new_ast_value(
                name,
                exception_data,
            )));

            Ok(FunctionCallResultSuccess::TailCall(TailCallData::new(
                handler,
                handler_env.as_shared(),
            )))
        } else {
            Ok(FunctionCallResultSuccess::Value(result.unwrap()))
        }
    }
}

struct ThrowFn;
impl NativeFunction for ThrowFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "throw".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let (mut ast, _env) = data.destructure();

        let exception = ast.remove(0);
        return Err(EvalError::CustomException(exception));
    }
}
