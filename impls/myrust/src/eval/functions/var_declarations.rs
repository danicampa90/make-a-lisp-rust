use std::rc::Rc;

use crate::eval::{Environment, EnvironmentEntry, EvalError};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(DefBang), Rc::new(LetStar)]
}

struct DefBang;
impl NativeFunction for DefBang {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "def!".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, env) = data.destructure();

        let name = params.remove(0).try_unwrap_symbol()?;
        let value = params.remove(0);

        let value = data.evaluator().eval(value, env.clone())?;
        env.get_root()
            .borrow_mut()
            .set_owned(EnvironmentEntry::new_ast_value(name, value.clone()));
        Ok(FunctionCallResultSuccess::Value(value))
    }
}

struct LetStar;
impl NativeFunction for LetStar {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "let*".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;
        let (mut params, env) = data.destructure();

        // get the binding list and do error checking there
        let mut bindings = params.remove(0).try_unwrap_list()?;

        if bindings.len() % 2 != 0 {
            return Err(EvalError::custom_exception_str(
                "let* binding list needs to be containing an even number of elements",
            ));
        }

        // uses a new environment
        let env = Environment::new_child(env.clone()).as_shared();
        let evaluator = data.evaluator();

        // evaluate with the help of def! on the new environment
        for _ in (0..bindings.len()).step_by(2) {
            let name = bindings.remove(0).try_unwrap_symbol()?;
            let value = bindings.remove(0);
            let value = evaluator.eval(value, env.clone())?;
            env.borrow_mut()
                .set_owned(EnvironmentEntry::new_ast_value(name, value));
        }

        let value = params.remove(0);
        Ok(FunctionCallResultSuccess::new_tailcall(value, env))
    }
}
