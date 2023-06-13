use crate::read::AstNode;

use super::{EnvironmentEntryValue, EvalError, SharedEnvironment};

// ////////////// Evaluator ////////////// //
pub struct Evaluator {}

impl Evaluator {
    fn eval_ast(&self, ast: AstNode, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
        Ok(match ast {
            AstNode::List(t) => {
                let r = t
                    .into_iter()
                    .map(|t| self.eval(t, env))
                    .collect::<Vec<Result<AstNode, EvalError>>>();
                let r = Result::from_iter(r)?;
                AstNode::List(r)
            }
            AstNode::Int(num) => AstNode::Int(num),
            AstNode::String(str) => AstNode::String(str),
            AstNode::UnresolvedSymbol(name) => env
                .borrow_mut()
                .lookup(&name)
                .ok_or(EvalError::SymbolNotFound(name))?
                .to_ast_node(),
            AstNode::FunctionPtr(fptr) => AstNode::FunctionPtr(fptr),
        })
    }

    pub fn eval(&self, ast: AstNode, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
        match ast {
            AstNode::List(empty) if empty.len() == 0 => Ok(AstNode::List(empty)),
            AstNode::List(mut list) => self.eval_funcall(list.remove(0), list, env),
            any => self.eval_ast(any, env),
        }
    }
    fn eval_funcall(
        &self,
        name: AstNode,
        mut params: Vec<AstNode>,
        env: &SharedEnvironment,
    ) -> Result<AstNode, EvalError> {
        if let AstNode::UnresolvedSymbol(name) = name {
            let definition = env
                .borrow()
                .lookup(&name)
                .ok_or_else(|| EvalError::SymbolNotFound(name.clone()))?;

            // evaluate the parameters in advance only if it's not a special form (or macro?)
            if definition.eval_parameters() {
                let resulting_list = self.eval_ast(AstNode::List(params), env)?;
                params = match resulting_list {
                    AstNode::List(l) => l,
                    _ => unreachable!(),
                }
            }
            // run
            match definition.value() {
                EnvironmentEntryValue::NativeFunction(func) => func(params, env),
                _ => Err(EvalError::NotACallableFunction(name)),
            }
        } else {
            Err(EvalError::InvalidFunctionCallNodeType(name))
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}
