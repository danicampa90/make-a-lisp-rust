use std::{cell::RefCell, rc::Rc};

use crate::read::AstNode;

use super::{Environment, EnvironmentEntry, EvalError, Evaluator, SharedEnvironment};

pub fn new_base_environment() -> SharedEnvironment {
    let mut env = Environment::new_root();
    /// math
    env.set_owned(EnvironmentEntry::new_native_function(
        "+".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a + b)),
    ));

    env.set_owned(EnvironmentEntry::new_native_function(
        "-".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a - b)),
    ));

    env.set_owned(EnvironmentEntry::new_native_function(
        "*".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a * b)),
    ));

    env.set_owned(EnvironmentEntry::new_native_function(
        "/".to_string(),
        |params, env| int_binary_operator(params, env, |a, b| (a / b)),
    ));

    /// variable def
    env.set_owned(EnvironmentEntry::new_special_atom(
        "def!".to_string(),
        def_impl,
    ));
    env.set_owned(EnvironmentEntry::new_special_atom(
        "let*".to_string(),
        letstar_impl,
    ));

    let global = Environment::new_child(env.as_shared());
    global.as_shared()
}

fn def_impl(mut params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    if params.len() != 2 {
        return Err(EvalError::custom_exception_str("def! expects 2 parameters"));
    }
    let name = params.remove(0);
    if let AstNode::UnresolvedSymbol(name) = name {
        let value = params.remove(0);
        let value = Evaluator::new().eval(value, env)?;
        env.borrow_mut()
            .set_owned(EnvironmentEntry::new_ast_value(name, value.clone()));
        Ok(value)
    } else {
        return Err(EvalError::custom_exception_str(
            "The first parameter of def! should be a symbol",
        ));
    }
}

fn letstar_impl(mut params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    if params.len() != 2 {
        return Err(EvalError::custom_exception_str("let* expects 2 parameters"));
    }
    let bindings = params.remove(0);
    let result = params.remove(0);
    if let AstNode::List(mut bindings) = bindings {
        if bindings.len() % 2 != 0 {
            return Err(EvalError::custom_exception_str(
                "let* binding list needs to be containing an even number of elements",
            ));
        }

        let env = Environment::new_child(env.clone()).as_shared();

        // evaluate with the help of def! on the new environment
        for i in (0..bindings.len()).step_by(2) {
            let name = bindings.remove(0);
            let value = bindings.remove(0);
            def_impl(vec![name, value], &env)?;
        }

        let value = Evaluator::new().eval(result, &env)?;
        return Ok(value);
    } else {
        return Err(EvalError::custom_exception_str(
            "The first parameter of def! should be a symbol",
        ));
    }
}

fn int_binary_operator(
    mut params: Vec<AstNode>,
    env: &SharedEnvironment,
    func: fn(i64, i64) -> i64,
) -> Result<AstNode, EvalError> {
    if params.len() != 2 {
        return Err(EvalError::custom_exception_str("Expected 2 parameters"));
    }
    let first = params.remove(0);
    let second = params.remove(0);

    match (first, second) {
        (AstNode::Int(a), AstNode::Int(b)) => Ok(AstNode::Int(func(a, b))),
        _ => Err(EvalError::custom_exception_str("Invalid argument types")),
    }
}
