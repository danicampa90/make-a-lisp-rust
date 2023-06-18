use std::{collections::HashMap, rc::Rc};

use crate::{
    eval::{EvalError, Evaluator, SharedEnvironment},
    read::AstNode,
};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(QuoteFn), Rc::new(QuasiQuoteFn)]
}

struct QuoteFn;
impl NativeFunction for QuoteFn {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "quote".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        Ok(FunctionCallResultSuccess::Value(
            data.destructure().0.remove(0),
        ))
    }
}
struct QuasiQuoteFn;
impl NativeFunction for QuasiQuoteFn {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "quasiquote".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let evaluator = data.evaluator();
        let (ast, env) = data.destructure();

        let mut ast = process_quasiquote(ast, &env, &evaluator)?;
        if ast.len() != 1 {
            return Err(EvalError::custom_exception_str(
                "quasiquote cannot return more than 1 argument",
            ));
        }

        Ok(FunctionCallResultSuccess::Value(ast.remove(0)))
    }
}

fn process_quasiquote(
    mut input_ast: Vec<AstNode>,
    env: &SharedEnvironment,
    evaluator: &Evaluator,
) -> Result<Vec<AstNode>, EvalError> {
    let mut result = vec![];
    while input_ast.len() > 0 {
        let mut expansion = match input_ast.remove(0) {
            AstNode::List(args) if args.len() > 0 => match &args[0] {
                AstNode::UnresolvedSymbol(name) if name == "unquote" => {
                    vec![quasiquote_run_unquote(args, env, evaluator)?]
                }
                AstNode::UnresolvedSymbol(name) if name == "splice-unquote" => {
                    let result = quasiquote_run_unquote(args, env, evaluator)?;
                    if let AstNode::List(result) = result {
                        result
                    } else {
                        Err(EvalError::custom_exception_str(
                            "splice-unquote expects a list as a result.",
                        ))?
                    }
                }
                _ => vec![AstNode::List(process_quasiquote(args, env, evaluator)?)],
            },
            AstNode::Vector(elements) => vec![AstNode::Vector(process_quasiquote(
                elements, env, evaluator,
            )?)],
            AstNode::HashMap(hm) => {
                let mut result = HashMap::new();
                for entry in hm {
                    let mut value = process_quasiquote(vec![entry.1], env, evaluator)?;
                    if value.len() != 1 {
                        Err(EvalError::custom_exception_str(
                            "Hashmap value has a splice-unquote",
                        ))?
                    }
                    result.insert(entry.0, value.remove(0));
                }
                vec![AstNode::HashMap(result)]
            }
            x => vec![x], // all other nodes are left as-is, as they are not inside an 'unquote' and they don't contain other ASTs
        };
        result.append(&mut expansion);
    }
    return Ok(result);
}

fn quasiquote_run_unquote(
    mut args: Vec<AstNode>,
    env: &SharedEnvironment,
    evaluator: &Evaluator,
) -> Result<AstNode, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::custom_exception_str(
            "unquote or splice-unquote only supports 1 argument",
        ));
    }

    evaluator.eval(args.remove(1), env.clone())
}
