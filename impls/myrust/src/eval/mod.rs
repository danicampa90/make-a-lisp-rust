mod errors;
mod evaluator;

pub use crate::environment::new_base_environment;
pub use crate::environment::{
    Environment, EnvironmentEntry, EnvironmentEntryValue, SharedEnvironment,
};
pub use errors::EvalError;
pub use evaluator::*;
