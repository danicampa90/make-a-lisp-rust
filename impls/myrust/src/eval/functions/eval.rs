use std::rc::Rc;

use super::{
    FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction, TailCallData,
};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![Rc::new(EvalFn), Rc::new(ReadStringFn)]
}

struct EvalFn;
impl NativeFunction for EvalFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "eval".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let (mut ast, env) = data.destructure();

        Ok(FunctionCallResultSuccess::TailCall(TailCallData::new(
            ast.remove(0),
            env,
        )))
    }
}

struct ReadStringFn;
impl NativeFunction for ReadStringFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "read-string".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        use crate::read::{InputReader, Lexer, Parser, StringInputSource};

        data.check_parameters_count_range(Some(1), Some(1))?;
        let (mut ast, _env) = data.destructure();

        let to_parse = ast.remove(0).try_unwrap_string()?;
        let mut input = InputReader::new(Box::new(StringInputSource::new(to_parse)));
        let lexer = Lexer::create_lexer_iterator(&mut input);
        let mut parser = Parser::new(lexer);

        match parser.read_form(true) {
            Ok(ast) => Ok(FunctionCallResultSuccess::Value(ast)),
            Err(parse_error) => Err(crate::eval::EvalError::CustomException(format!(
                "eval error: {:?}",
                parse_error
            ))),
        }
    }
}
