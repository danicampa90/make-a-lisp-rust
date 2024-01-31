use crate::read::{AstNode, AstNodeRef};

use crate::eval::{EvalError, Evaluator, SharedEnvironment};

pub struct TailCallData {
    to_eval: AstNode,
    env: SharedEnvironment,
}

impl TailCallData {
    pub fn destructure(self) -> (AstNode, SharedEnvironment) {
        (self.to_eval, self.env)
    }
    pub fn new(to_eval: AstNode, env: SharedEnvironment) -> TailCallData {
        Self { to_eval, env }
    }
}

pub enum FunctionCallResultSuccess {
    Value(AstNodeRef),
    TailCall(TailCallData),
}
impl FunctionCallResultSuccess {
    pub fn new_tailcall(to_eval: AstNode, env: SharedEnvironment) -> FunctionCallResultSuccess {
        Self::TailCall(TailCallData::new(to_eval, env))
    }
}

pub type FunctionCallResult = Result<FunctionCallResultSuccess, EvalError>;

pub struct FunctionCallData {
    call_context: Option<(Vec<AstNode> /* params */, SharedEnvironment)>,
    evaluator: Evaluator,
}
impl FunctionCallData {
    pub fn params(&self) -> &Vec<AstNode> {
        &self.call_context.as_ref().unwrap().0
    }
    pub fn destructure(&mut self) -> (Vec<AstNode>, SharedEnvironment) {
        self.call_context.take().unwrap()
    }
    pub fn evaluator(&self) -> Evaluator {
        self.evaluator.clone()
    }
    pub fn check_parameters_count_range(
        &self,
        expected_min: Option<usize>,
        expected_max: Option<usize>,
    ) -> Result<(), EvalError> {
        let len = self.params().len();
        if let Some(min) = expected_min {
            if len < min {
                return Err(EvalError::ParameterCountError {
                    expected_min: expected_min,
                    expected_max: expected_max,
                    provided: len,
                });
            }
        }
        if let Some(max) = expected_max {
            if len > max {
                return Err(EvalError::ParameterCountError {
                    expected_min: expected_min,
                    expected_max: expected_max,
                    provided: len,
                });
            }
        }
        return Ok(());
    }
}

pub trait NativeFunction {
    fn evaluates_arguments(&self) -> bool;
    fn name(&self) -> String;
    fn run(&self, data: FunctionCallData) -> FunctionCallResult;
}

impl dyn NativeFunction {
    pub fn eval_params_and_run(
        &self,
        mut params: Vec<AstNode>,
        env: SharedEnvironment,
        evaluator: Evaluator,
    ) -> FunctionCallResult {
        if self.evaluates_arguments() {
            // evaluate the parameters in advance only if it's not a special form (or macro?)
            let r = params.into_iter().map(|t| evaluator.eval(t, env.clone()));
            params = Result::from_iter(r)?
        }

        evaluator.trace_native_funcall(self, &params);
        self.run(FunctionCallData {
            call_context: Some((params, env)),
            evaluator: evaluator,
        })
    }
}
