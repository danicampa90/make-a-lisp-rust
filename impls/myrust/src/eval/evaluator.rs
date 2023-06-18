use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use crate::read::AstNode;

use super::{
    Environment, EnvironmentEntry, EnvironmentEntryValue, EvalError, FunctionCallResult,
    FunctionCallResultSuccess, NativeFunction, SharedEnvironment,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[repr(u8)]
pub enum TraceFlag {
    TraceNativeFunctionCalls = 1,
    TraceFnCalls = 2,
}

pub struct EvaluatorData {
    trace_config: BTreeSet<TraceFlag>,
}
impl EvaluatorData {
    fn new() -> EvaluatorData {
        EvaluatorData {
            trace_config: BTreeSet::new(),
        }
    }
}

#[derive(Clone)]
pub struct Evaluator {
    data: Rc<RefCell<EvaluatorData>>,
}

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
            AstNode::Vector(content) => AstNode::Vector(Result::from_iter(
                content.into_iter().map(|a| self.eval(a, env.clone())),
            )?),
            AstNode::Atom(rc) => AstNode::Atom(rc),
            AstNode::HashMap(content) => {
                let iter_kv = Result::from_iter(content.into_iter().map(
                    |a| -> Result<(String, AstNode), EvalError> {
                        Ok((a.0, self.eval(a.1, env.clone())?))
                    },
                ))?;

                AstNode::HashMap(iter_kv)
            }
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
        params: Vec<AstNode>,
        env: SharedEnvironment,
    ) -> FunctionCallResult {
        let func = self.eval(func, env.clone())?;

        match func {
            AstNode::FunctionPtr(definition) => {
                // run
                match definition.value() {
                    EnvironmentEntryValue::NativeFunction(func) => func.eval_params_and_run(params, env, self.clone()),
                    EnvironmentEntryValue::Value(_) => unreachable!("This should never be a value, as those get evaluated out into actual AST nodes in the evaluator")
                }
            }
            AstNode::Lambda(definition) => {
                let lambda = &*definition;

                let mut new_env = Environment::new_child(lambda.env.clone());

                if params.len() != lambda.params.len() {
                    return Err(EvalError::custom_exception_str(format!(
                        "Function application expected {} parameters, but found {} instead",
                        lambda.params.len(),
                        params.len()
                    )));
                }

                for i in 0..params.len() {
                    // cloning values here is not super-great
                    let value = self.eval(params[i].clone(), env.clone())?;
                    let name = lambda.params[i].clone();
                    new_env.set_owned(EnvironmentEntry::new_ast_value(name, value))
                }

                self.trace_lambda_funcall(&lambda.body, &lambda.params, &new_env);

                // TODO: Cloning the AST for evaluating it is terribly inefficient! Use references instead.
                if !lambda.is_macro {
                    // normal function application means we can actually tailcall it
                    Ok(FunctionCallResultSuccess::new_tailcall(
                        lambda.body.clone(),
                        new_env.as_shared(),
                    ))
                } else {
                    let result = self.eval(lambda.body.clone(), new_env.as_shared())?;
                    Ok(FunctionCallResultSuccess::new_tailcall(result, env))
                }
            }
            node => Err(EvalError::InvalidFunctionCallNodeType(node)),
        }
    }

    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(EvaluatorData::new())),
        }
    }

    pub fn set_trace(&self, flag: TraceFlag, enabled: bool) {
        if enabled {
            self.data.borrow_mut().trace_config.insert(flag);
        } else {
            self.data.borrow_mut().trace_config.remove(&flag);
        }
    }

    pub fn is_tracing(&self, flag: &TraceFlag) -> bool {
        self.data.borrow().trace_config.contains(flag)
    }

    pub(crate) fn trace_native_funcall(
        &self,
        function: &dyn NativeFunction,
        params: &Vec<AstNode>,
    ) {
        if self.is_tracing(&TraceFlag::TraceNativeFunctionCalls) {
            println!(
                "calling native function ({0} {1})",
                function.name(),
                params
                    .iter()
                    .map(|n| format!("{:?}", n))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }

    fn trace_lambda_funcall(&self, body: &AstNode, params_names: &Vec<String>, env: &Environment) {
        if self.is_tracing(&TraceFlag::TraceFnCalls) {
            println!("calling fn*:");
            println!("  params:");
            for n in params_names {
                match env.find(n) {
                    Some(v) => match v.value() {
                        _ => println!("    {} = {:?}", n, 4),
                    },
                    None => println!("    {} = <unbound!>", n),
                }
            }

            println!("  body:{:?}", body);
        }
    }
}
