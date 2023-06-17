use crate::read::AstNode;

use super::{
    Environment, EnvironmentEntry, EnvironmentEntryValue, EvalError, FunctionCallData,
    FunctionCallResult, FunctionCallResultSuccess, SharedEnvironment, TailCallData,
};

// ////////////// Evaluator ////////////// //

#[derive(Clone)]
pub struct Evaluator {}

impl Evaluator {
    fn eval_ast(&self, ast: AstNode, env: &SharedEnvironment) -> FunctionCallResult {
        Ok(FunctionCallResultSuccess::Value(match ast {
            AstNode::List(_) => unreachable!(),
            AstNode::Int(num) => AstNode::Int(num),
            AstNode::String(str) => AstNode::String(str),
            AstNode::Bool(b) => AstNode::Bool(b),
            AstNode::Nil => AstNode::Nil,
            AstNode::UnresolvedSymbol(name) => env
                .borrow()
                .find(&name)
                .ok_or(EvalError::SymbolNotFound(name))?
                .to_ast_node(),
            AstNode::FunctionPtr(fptr) => AstNode::FunctionPtr(fptr),
            AstNode::Lambda(l) => AstNode::Lambda(l),
        }))
    }

    pub fn eval(&self, mut ast: AstNode, mut env: SharedEnvironment) -> Result<AstNode, EvalError> {
        loop {
            let tailcall_result = match ast {
                AstNode::List(empty) if empty.len() == 0 => {
                    Ok(FunctionCallResultSuccess::Value(AstNode::List(empty)))
                }
                AstNode::List(mut list) => self.eval_funcall(list.remove(0), list, env),
                any => self.eval_ast(any, &env),
            }?;

            match tailcall_result {
                FunctionCallResultSuccess::Value(v) => return Ok(v),
                FunctionCallResultSuccess::TailCall(tailcalldata) => {
                    let (new_ast, new_env) = tailcalldata.destructure();
                    ast = new_ast;
                    env = new_env;
                }
            }
        }
    }
    fn eval_funcall(
        &self,
        func: AstNode,
        mut params: Vec<AstNode>,
        env: SharedEnvironment,
    ) -> FunctionCallResult {
        let evaluator = Evaluator::new();
        let func = evaluator.eval(func, env.clone())?;

        match func {
            AstNode::FunctionPtr(definition) => {
                // run
                match definition.value() {
                    EnvironmentEntryValue::NativeFunction(func) => func.eval_params_and_run(params, env, self.clone()),
                    EnvironmentEntryValue::Value(_) => unreachable!("This should never be a value, as those get evaluated out into actual AST nodes in the evaluator")
                }
            }
            AstNode::Lambda(definition) => {
                let (lambda_params, lambda_body, captured_env) = &*definition;

                let mut new_env = Environment::new_child(captured_env.clone());

                if params.len() != lambda_params.len() {
                    return Err(EvalError::custom_exception_str(format!(
                        "Function application expected {} parameters, but found {} instead",
                        lambda_params.len(),
                        params.len()
                    )));
                }

                for i in 0..params.len() {
                    // cloning values here is not super-great
                    let value = evaluator.eval(params[i].clone(), env.clone())?;
                    let name = lambda_params[i].clone();
                    new_env.set_owned(EnvironmentEntry::new_ast_value(name, value))
                }

                // TODO: Cloning the AST for evaluating it is terribly inefficient! Use references instead.
                Ok(FunctionCallResultSuccess::new_tailcall(
                    lambda_body.clone(),
                    new_env.as_shared(),
                ))
            }
            node => Err(EvalError::InvalidFunctionCallNodeType(node)),
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}
