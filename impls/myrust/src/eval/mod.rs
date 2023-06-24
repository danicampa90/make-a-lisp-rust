mod environment;
mod errors;
mod evaluator;
mod functions;

pub use environment::new_base_environment;
pub use environment::{Environment, EnvironmentEntry, EnvironmentEntryValue, SharedEnvironment};
pub use errors::EvalError;
pub use evaluator::*;
pub use functions::*;
