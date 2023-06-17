use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(ListFn),
        Rc::new(IsListFn),
        Rc::new(CountFn),
        Rc::new(ConsFn),
        Rc::new(ConcatFn),
    ]
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

struct ConsFn;
impl NativeFunction for ConsFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "cons".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;
        let mut ast = data.destructure().0;
        let value = ast.remove(0);
        let mut list = ast.remove(0).try_unwrap_list_or_vector()?;
        list.insert(0, value);

        Ok(FunctionCallResultSuccess::Value(AstNode::List(list)))
    }
}

struct ConcatFn;
impl NativeFunction for ConcatFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "concat".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        let mut ast = data.destructure().0;
        if ast.len() == 0 {
            return Ok(FunctionCallResultSuccess::Value(AstNode::List(vec![])));
        }

        let mut list = ast.remove(0).try_unwrap_list_or_vector()?;
        while ast.len() > 0 {
            let mut to_append = ast.remove(0).try_unwrap_list_or_vector()?;
            list.append(&mut to_append);
        }

        Ok(FunctionCallResultSuccess::Value(AstNode::List(list)))
    }
}
