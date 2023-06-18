use std::rc::Rc;

use crate::{
    eval::{EnvironmentEntry, EvalError},
    read::AstNode,
};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(DefMacroFn)]
}

struct DefMacroFn;
impl NativeFunction for DefMacroFn {
    fn evaluates_arguments(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        "defmacro!".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut params, env) = data.destructure();

        let name = params.remove(0).try_unwrap_symbol()?;
        let value = params.remove(0);

        let value = data.evaluator().eval(value, env.clone())?;
        let value = match value {
            AstNode::Lambda(func) => AstNode::Lambda(Rc::new((*func).clone().set_is_macro(true))),
            x => {
                return Err(EvalError::TypeError {
                    expected: "fn* lambda".to_string(),
                    got: x,
                })
            }
        };

        env.get_root()
            .borrow_mut()
            .set_owned(EnvironmentEntry::new_ast_value(name, value.clone()));
        Ok(FunctionCallResultSuccess::Value(value))
    }
}
