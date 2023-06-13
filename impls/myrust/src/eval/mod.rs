mod environment;
mod errors;
mod evaluator;
mod predefined_functions;

pub use environment::{Environment, EnvironmentEntry, EnvironmentEntryValue, SharedEnvironment};
pub use errors::EvalError;
pub use evaluator::*;
pub use predefined_functions::new_base_environment;
