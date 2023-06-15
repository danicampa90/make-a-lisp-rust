use std::rc::Rc;

use crate::read::{AstNode, AstPrintFormat, AstPrinter};

use super::{evaluator, Environment, EnvironmentEntry, EvalError, Evaluator, SharedEnvironment};

pub fn new_base_environment() -> SharedEnvironment {
    let mut env = Environment::new_root();
    // Step 2: math
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

    // Step 3: variable def
    env.set_owned(EnvironmentEntry::new_special_atom(
        "def!".to_string(),
        def_impl,
    ));
    env.set_owned(EnvironmentEntry::new_special_atom(
        "let*".to_string(),
        letstar_impl,
    ));

    // Step 4: Booleans & nil
    env.set_owned(EnvironmentEntry::new_ast_value(
        "true".to_string(),
        AstNode::Bool(true),
    ));
    env.set_owned(EnvironmentEntry::new_ast_value(
        "false".to_string(),
        AstNode::Bool(false),
    ));
    env.set_owned(EnvironmentEntry::new_ast_value(
        "nil".to_string(),
        AstNode::Nil,
    ));

    // Step 4: control flow & do
    env.set_owned(EnvironmentEntry::new_special_atom(
        "if".to_string(),
        if_impl,
    ));
    env.set_owned(EnvironmentEntry::new_special_atom(
        "do".to_string(),
        do_impl,
    ));
    env.set_owned(EnvironmentEntry::new_special_atom(
        "fn*".to_string(),
        fnstar_impl,
    ));

    // Step 4: String functions
    env.set_owned(EnvironmentEntry::new_native_function(
        "prn".to_string(),
        |params, _env| {
            println!("{}", ast_to_str(params, " ", AstPrintFormat::Repr));
            Ok(AstNode::Nil)
        },
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "println".to_string(),
        |params, _env| {
            println!("{}", ast_to_str(params, " ", AstPrintFormat::Readable));
            Ok(AstNode::Nil)
        },
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "pr-str".to_string(),
        |params, _env| {
            Ok(AstNode::String(ast_to_str(
                params,
                " ",
                AstPrintFormat::Repr,
            )))
        },
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "str".to_string(),
        |params, _env| {
            Ok(AstNode::String(ast_to_str(
                params,
                "",
                AstPrintFormat::Readable,
            )))
        },
    ));
    // Step 4: List ops
    env.set_owned(EnvironmentEntry::new_native_function(
        "list".to_string(),
        |params, _env| Ok(AstNode::List(params)),
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "list?".to_string(),
        |params, _env| {
            check_param_count(&params, 1, "list?")?;
            Ok(AstNode::Bool(match params[0] {
                AstNode::List(_) => true,
                _ => false,
            }))
        },
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "count".to_string(),
        |params, _env| {
            check_param_count(&params, 1, "count")?;
            match &params[0] {
                AstNode::List(l) => Ok(AstNode::Int(l.len() as i64)),
                _ => Ok(AstNode::Int(0)),
            }
        },
    ));

    // Step 4: Bool operators & boolean logic
    env.set_owned(EnvironmentEntry::new_native_function(
        "=".to_string(),
        |params, _env| {
            check_param_count(&params, 2, "=")?;
            Ok(AstNode::Bool(params[0] == params[1]))
        },
    ));

    env.set_owned(EnvironmentEntry::new_native_function(
        "<".to_string(),
        |params, _env| {
            check_param_count(&params, 2, "<")?;
            match (&params[0], &params[1]) {
                (AstNode::Int(a), AstNode::Int(b)) => Ok(AstNode::Bool(a < b)),
                _ => Err(EvalError::custom_exception_str(
                    "< has some invalid parameter types",
                )),
            }
        },
    ));
    env.set_owned(EnvironmentEntry::new_native_function(
        "nand".to_string(),
        |params, _env| {
            check_param_count(&params, 2, "nand")?;
            match (&params[0], &params[1]) {
                (AstNode::Bool(a), AstNode::Bool(b)) => Ok(AstNode::Bool(!(*a && *b))),
                _ => Err(EvalError::custom_exception_str(
                    "or has some invalid parameter types",
                )),
            }
        },
    ));

    let global = Environment::new_child(env.as_shared());
    global.as_shared()
}

fn check_param_count(
    params: &Vec<AstNode>,
    expected: usize,
    fn_name: &str,
) -> Result<(), EvalError> {
    if params.len() != expected {
        return Err(EvalError::custom_exception_str(format!(
            "{} expects {} parameters",
            fn_name, expected
        )));
    } else {
        return Ok(());
    }
}

fn int_binary_operator(
    mut params: Vec<AstNode>,
    _env: &SharedEnvironment,
    func: fn(i64, i64) -> i64,
) -> Result<AstNode, EvalError> {
    check_param_count(&params, 2, "binary numeric operator")?;

    let first = params.remove(0);
    let second = params.remove(0);

    match (first, second) {
        (AstNode::Int(a), AstNode::Int(b)) => Ok(AstNode::Int(func(a, b))),
        _ => Err(EvalError::custom_exception_str("Invalid argument types")),
    }
}

fn def_impl(mut params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    check_param_count(&params, 2, "def!")?;

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
    check_param_count(&params, 2, "let*")?;

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
        for _ in (0..bindings.len()).step_by(2) {
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

fn if_impl(mut params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    if params.len() != 2 && params.len() != 3 {
        return Err(EvalError::custom_exception_str(
            "if expects 2 or 3 parameters",
        ));
    }

    let evaluator = Evaluator::new();
    let condition = params.remove(0);
    let truebranch = params.remove(0);
    let falsebranch = if params.len() > 0 {
        params.remove(0)
    } else {
        AstNode::Nil
    };
    let condition_value = evaluator.eval(condition, env)?;
    match condition_value {
        AstNode::Bool(false) => evaluator.eval(falsebranch, env),
        AstNode::Nil => evaluator.eval(falsebranch, env),
        _ => evaluator.eval(truebranch, env),
    }
}

fn do_impl(params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    let evaluator = Evaluator::new();
    let mut final_value = AstNode::Nil;
    for param in params.into_iter() {
        final_value = evaluator.eval(param, env)?;
    }
    Ok(final_value)
}

fn fnstar_impl(mut params: Vec<AstNode>, env: &SharedEnvironment) -> Result<AstNode, EvalError> {
    check_param_count(&params, 2, "fn*")?;

    let lambda_params = params.remove(0);
    let lambda_body = params.remove(0);
    if let AstNode::List(lambda_params) = lambda_params {
        let mut params_as_strings = vec![];
        for lambda_param in lambda_params.into_iter() {
            match lambda_param {
                AstNode::UnresolvedSymbol(name) => params_as_strings.push(name),
                _ => {
                    return Err(EvalError::custom_exception_str(
                        "fn* parameters should be only symbols",
                    ))
                }
            }
        }

        return Ok(AstNode::Lambda(Rc::new((
            params_as_strings,
            lambda_body,
            env.clone(),
        ))));
    } else {
        Err(EvalError::custom_exception_str(
            "fn* first parameters should be a list",
        ))
    }
}

fn ast_to_str(params: Vec<AstNode>, separator: &str, format: AstPrintFormat) -> String {
    let mut builder = string_builder::Builder::new(64);
    let mut first_print = true;
    let printer = AstPrinter::new(format);
    for ast in params.into_iter() {
        let str = printer.ast_to_string(&ast);
        if first_print {
            builder.append(str);
        } else {
            builder.append(separator);
            builder.append(str);
        }
        first_print = false;
    }
    return builder.string().unwrap();
}
