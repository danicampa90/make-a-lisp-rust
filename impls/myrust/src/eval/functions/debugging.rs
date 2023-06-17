use std::rc::Rc;

use crate::{eval::TraceFlag, read::AstNode};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(SetTraceFlagFn::new(TraceFlag::TraceNativeFunctionCalls)),
        Rc::new(SetTraceFlagFn::new(TraceFlag::TraceFnCalls)),
    ]
}

struct SetTraceFlagFn {
    flag: TraceFlag,
}
impl SetTraceFlagFn {
    fn new(flag: TraceFlag) -> SetTraceFlagFn {
        SetTraceFlagFn { flag }
    }
}

impl NativeFunction for SetTraceFlagFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "set-trace-".to_string()
            + match self.flag {
                TraceFlag::TraceNativeFunctionCalls => "native-calls",
                TraceFlag::TraceFnCalls => "calls",
            }
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        let evaluator = data.evaluator();
        let (mut ast, _env) = data.destructure();

        let enabled = ast.remove(0).try_unwrap_bool()?;
        evaluator.set_trace(self.flag, enabled);
        Ok(FunctionCallResultSuccess::Value(AstNode::Nil))
    }
}
