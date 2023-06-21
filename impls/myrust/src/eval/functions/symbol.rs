use std::rc::Rc;

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(SymbolFn),
        Rc::new(IsSymbolFn),
        Rc::new(KeywordFn),
        Rc::new(IsKeywordFn),
    ]
}

struct SymbolFn;
impl NativeFunction for SymbolFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "symbol".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::UnresolvedSymbol(
            data.destructure().0.remove(0).try_unwrap_string()?,
        )))
    }
}

struct IsSymbolFn;
impl NativeFunction for IsSymbolFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "symbol?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            data.destructure().0.remove(0).try_unwrap_symbol().is_ok(),
        )))
    }
}

struct KeywordFn;
impl NativeFunction for KeywordFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "keyword".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::create_keyword(
            &data.destructure().0.remove(0).try_unwrap_string()?,
        )))
    }
}

struct IsKeywordFn;
impl NativeFunction for IsKeywordFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "keyword?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;

        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            data.destructure().0.remove(0).try_unwrap_keyword().is_ok(),
        )))
    }
}
