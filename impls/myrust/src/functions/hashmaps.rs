use std::rc::Rc;

use crate::{eval::EvalError, read::AstNode};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(IsMapFn),
        Rc::new(AssocFn),
        Rc::new(DissocFn),
        Rc::new(GetFn),
        Rc::new(ContainsFn),
        Rc::new(KeysValsFn::Keys),
        Rc::new(KeysValsFn::Vals),
    ]
}

struct IsMapFn;
impl NativeFunction for IsMapFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "map?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        let ast = data.destructure().0.remove(0);
        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            ast.try_unwrap_hashmap().is_ok(),
        )))
    }
}

struct AssocFn;
impl NativeFunction for AssocFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "assoc".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), None)?;

        let mut ast = data.destructure().0;

        let hashmap = ast.remove(0);
        if hashmap == AstNode::Nil {
            // some tests were checking that if hashmap is nil then the result is nil
            // this behavior was completely undocumented in the specs :(
            return Ok(FunctionCallResultSuccess::Value(AstNode::Nil));
        }
        let mut hashmap = hashmap.try_unwrap_hashmap()?;

        while ast.len() != 0 {
            if ast.len() == 1 {
                return Err(EvalError::custom_exception_str(
                    "Expected an even number of parameters for hash-map",
                ));
            }
            let key = ast.remove(0).try_unwrap_string()?;
            let value = ast.remove(0);
            hashmap.insert(key, value);
        }

        Ok(FunctionCallResultSuccess::Value(AstNode::HashMap(hashmap)))
    }
}

struct DissocFn;
impl NativeFunction for DissocFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "dissoc".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), None)?;

        let mut ast = data.destructure().0;

        let hashmap = ast.remove(0);
        if hashmap == AstNode::Nil {
            // some tests were checking that if hashmap is nil then the result is nil
            // this behavior was completely undocumented in the specs :(
            return Ok(FunctionCallResultSuccess::Value(AstNode::Nil));
        }
        let mut hashmap = hashmap.try_unwrap_hashmap()?;

        while ast.len() != 0 {
            let key = ast.remove(0).try_unwrap_string()?;
            hashmap.remove(&key) /* ignore missing values */;
        }

        Ok(FunctionCallResultSuccess::Value(AstNode::HashMap(hashmap)))
    }
}

struct GetFn;
impl NativeFunction for GetFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "get".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let mut ast = data.destructure().0;

        let hashmap = ast.remove(0);
        if hashmap == AstNode::Nil {
            // some tests were checking that if hashmap is nil then the result is nil
            // this behavior was completely undocumented in the specs :(
            return Ok(FunctionCallResultSuccess::Value(AstNode::Nil));
        }
        let mut hashmap = hashmap.try_unwrap_hashmap()?;

        let key = ast.remove(0).try_unwrap_string()?;

        let value = hashmap.remove(&key).unwrap_or(AstNode::Nil);

        Ok(FunctionCallResultSuccess::Value(value))
    }
}

struct ContainsFn;
impl NativeFunction for ContainsFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "contains?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;

        let mut ast = data.destructure().0;

        let hashmap = ast.remove(0);
        if hashmap == AstNode::Nil {
            // be consistent with the behavior of get - never raise errors when hashmap is nil
            return Ok(FunctionCallResultSuccess::Value(AstNode::Bool(false)));
        }
        let hashmap = hashmap.try_unwrap_hashmap()?;

        let key = ast.remove(0).try_unwrap_string()?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            hashmap.contains_key(&key),
        )))
    }
}

enum KeysValsFn {
    Keys,
    Vals,
}
impl NativeFunction for KeysValsFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        match self {
            Self::Keys => "keys",
            Self::Vals => "vals",
        }
        .to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        let mut ast = data.destructure().0;

        let hashmap = ast.remove(0);
        if hashmap == AstNode::Nil {
            // be consistent with the behavior of get - never raise errors when hashmap is nil
            return Ok(FunctionCallResultSuccess::Value(AstNode::List(vec![])));
        }
        let hashmap = hashmap.try_unwrap_hashmap()?;

        Ok(FunctionCallResultSuccess::Value(AstNode::List(
            hashmap
                .into_iter()
                .map(|kv| match self {
                    Self::Keys => AstNode::String(kv.0),
                    Self::Vals => kv.1,
                })
                .collect(),
        )))
    }
}
