use std::{cell::RefCell, rc::Rc};

use crate::read::AstNode;

use super::{Environment, EnvironmentEntry, EvalError, SharedEnvironment};

pub fn new_base_environment() -> SharedEnvironment {
    let mut env = Environment::new();
    env.add_entry_owned(EnvironmentEntry::new_native_function(
        "+".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a + b)),
    ));

    env.add_entry_owned(EnvironmentEntry::new_native_function(
        "-".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a - b)),
    ));

    env.add_entry_owned(EnvironmentEntry::new_native_function(
        "*".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a * b)),
    ));

    env.add_entry_owned(EnvironmentEntry::new_native_function(
        "/".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a / b)),
    ));

    Rc::new(RefCell::new(env))
}

fn int_binary_operator(
    mut params: Vec<AstNode>,
    env: &SharedEnvironment,
    func: fn(i64, i64) -> i64,
) -> Result<AstNode, EvalError> {
    if params.len() != 2 {
        return Err(EvalError::custom_exception_string("Expected 2 parameters"));
    }
    let first = params.remove(0);
    let second = params.remove(0);

    match (first, second) {
        (AstNode::Int(a), AstNode::Int(b)) => Ok(AstNode::Int(func(a, b))),
        _ => Err(EvalError::custom_exception_string("Invalid argument types")),
    }
}
