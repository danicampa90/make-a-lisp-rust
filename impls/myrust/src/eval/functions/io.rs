// in addition to i/o functions, it contains other "platform" routines like time management, metadata association,

use std::{io::Write, rc::Rc};

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(ReadLineFn),
        Rc::new(HostLanguageFn),
        Rc::new(ArgvFn),
    ]
}

struct ReadLineFn;
impl NativeFunction for ReadLineFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "readline".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(0), Some(1))?;
        let mut args = data.destructure().0;
        if args.len() == 1 {
            print!("{}", args.remove(0).try_unwrap_string()?);
            std::io::stdout().flush().unwrap();
        }
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if buf.ends_with('\n') {
            buf.remove(buf.len() - 1);
        }
        Ok(FunctionCallResultSuccess::Value(AstNode::String(buf)))
    }
}
struct HostLanguageFn;
impl NativeFunction for HostLanguageFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "*host-language*".to_string()
    }

    fn run(&self, data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(0), Some(0))?;
        Ok(FunctionCallResultSuccess::Value(AstNode::String(
            "myrust".to_string(),
        )))
    }
}

struct ArgvFn;
impl NativeFunction for ArgvFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "get-argv".to_string()
    }

    fn run(&self, data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(0), Some(0))?;

        let args = std::env::args().map(|arg| AstNode::String(arg)).collect();
        Ok(FunctionCallResultSuccess::Value(AstNode::List(args)))
    }
}
