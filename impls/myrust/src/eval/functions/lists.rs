use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(ListFn), Rc::new(IsListFn), Rc::new(CountFn)]
}

struct ListFn;
impl NativeFunction for ListFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "list".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        Ok(FunctionCallResultSuccess::Value(AstNode::List(
            data.destructure().0,
        )))
    }
}

struct IsListFn;
impl NativeFunction for IsListFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "list?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            data.destructure().0.remove(0).try_unwrap_list().is_ok(),
        )))
    }
}
struct CountFn;
impl NativeFunction for CountFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "count".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Int(
            data.destructure()
                .0
                .remove(0)
                .try_unwrap_list_or_vector()
                .map(|l| l.len())
                .unwrap_or(0) as i64,
        )))
    }
}
