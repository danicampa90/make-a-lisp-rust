use std::f32::consts::E;

use crate::read::AstNode;

use super::{
    environment, Environment, EnvironmentEntry, EnvironmentEntryValue, EvalError, SharedEnvironment,
};

// ////////////// Evaluator ////////////// //
pub struct Evaluator {}

impl Evaluator {
    fn eval_ast(&self, ast: AstNode, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
        Ok(match ast {
            AstNode::List(t) => unreachable!(),
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
        })
    }

    pub fn eval(&self, ast: AstNode, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
        match ast {
            AstNode::List(empty) if empty.len() == 0 => Ok(AstNode::List(empty)),
            AstNode::List(mut list) => self.eval_funcall(list.remove(0), list, env),
            any => self.eval_ast(any, env),
        }
    }
    pub fn eval_parameter_list(
        &self,
        list: Vec<AstNode>,
        env: &SharedEnvironment,
    ) -> Result<Vec<AstNode>, EvalError> {
        let r = list.into_iter().map(|t| self.eval(t, env));
        Result::from_iter(r)
    }
    fn eval_funcall(
        &self,
        func: AstNode,
        mut params: Vec<AstNode>,
        env: &SharedEnvironment,
    ) -> Result<AstNode, EvalError> {
        let evaluator = Evaluator::new();
        let func = evaluator.eval(func, env)?;

        match func {
            AstNode::FunctionPtr(definition) => {
                // evaluate the parameters in advance only if it's not a special form (or macro?)
                if definition.eval_parameters() {
                    params = self.eval_parameter_list(params, env)?
                }
                // run
                match definition.value() {
                    EnvironmentEntryValue::NativeFunction(func) => func(params, env),
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
                    // cloning values here is also not super-great, but they should never be "big" anyway (maybe only for lists???)
                    let value = evaluator.eval(params[i].clone(), env)?;
                    let name = lambda_params[i].clone();
                    println!("set {} = {:?}", name, value);
                    new_env.set_owned(EnvironmentEntry::new_ast_value(name, value))
                }

                // TODO: Cloning the AST for evaluating it is terribly inefficient! Use references instead.
                evaluator.eval(lambda_body.clone(), &new_env.as_shared())
            }
            node => Err(EvalError::InvalidFunctionCallNodeType(node)),
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}
