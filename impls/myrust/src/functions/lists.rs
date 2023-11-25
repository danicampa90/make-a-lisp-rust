use std::rc::Rc;

use crate::{eval::EvalError, read::AstNode};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(ListFn),
        Rc::new(IsListFn),
        Rc::new(CountFn),
        Rc::new(NthFn),
        Rc::new(RestFn),
        Rc::new(ConsFn),
        Rc::new(ConcatFn),
        Rc::new(VecFn),
        Rc::new(IsVectorFn),
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

struct IsVectorFn;
impl NativeFunction for IsVectorFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "vector?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            data.destructure().0.remove(0).try_unwrap_vector().is_ok(),
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
struct NthFn;
impl NativeFunction for NthFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "nth".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let (mut ast, _env) = data.destructure();
        let mut list = ast.remove(0).try_unwrap_list_or_vector()?;
        let index = ast.remove(0).try_unwrap_int()? as usize;

        if list.len() > index {
            Ok(FunctionCallResultSuccess::Value(list.remove(index)))
        } else {
            Err(EvalError::custom_exception_str("index out of range in nth"))
        }
    }
}
struct RestFn;
impl NativeFunction for RestFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "rest".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        let (mut ast, _env) = data.destructure();

        let res = match ast.remove(0) {
            AstNode::List(mut l) if l.len() > 0 => {
                l.remove(0);
                l
            }
            AstNode::Vector(mut l) if l.len() > 0 => {
                l.remove(0);
                l
            }

            AstNode::List(_) => vec![],
            AstNode::Vector(_) => vec![],
            AstNode::Nil => vec![],
            x => {
                return Err(EvalError::TypeError {
                    expected: "List/Vector/Nil".to_string(),
                    got: x,
                })
            }
        };
        Ok(FunctionCallResultSuccess::Value(AstNode::List(res)))
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

struct VecFn;
impl NativeFunction for VecFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "vec".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let elements = match data.destructure().0.remove(0) {
            AstNode::List(elements) => elements,
            AstNode::Vector(elements) => elements,
            x => {
                return Err(EvalError::TypeError {
                    expected: "List/Vector".to_string(),
                    got: x,
                })
            }
        };
        Ok(FunctionCallResultSuccess::Value(AstNode::Vector(elements)))
    }
}
