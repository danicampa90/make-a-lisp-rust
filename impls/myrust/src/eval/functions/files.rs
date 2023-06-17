use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(SlurpFn)]
}

struct SlurpFn;
impl NativeFunction for SlurpFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "slurp".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let (mut ast, _env) = data.destructure();
        let filename = ast.remove(0).try_unwrap_string()?;

        match std::fs::read_to_string(filename) {
            Ok(contents) => Ok(FunctionCallResultSuccess::Value(AstNode::String(contents))),
            Err(_err) => Ok(FunctionCallResultSuccess::Value(AstNode::Nil)),
        }
    }
}
