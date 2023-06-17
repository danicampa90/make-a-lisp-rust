use crate::read::AstNode;

use super::{Environment, EnvironmentEntry, SharedEnvironment};

pub fn new_base_environment() -> SharedEnvironment {
    let mut env = Environment::new_root();
    // Step 4: Booleans & nil
    env.set_owned(EnvironmentEntry::new_ast_value(
        "true".to_string(),
        AstNode::Bool(true),
    ));
    env.set_owned(EnvironmentEntry::new_ast_value(
        "false".to_string(),
        AstNode::Bool(false),
    ));
    env.set_owned(EnvironmentEntry::new_ast_value(
        "nil".to_string(),
        AstNode::Nil,
    ));

    for func in super::functions::global_functions() {
        env.set_owned(EnvironmentEntry::new_native(func));
    }

    let global = Environment::new_child(env.as_shared());
    global.as_shared()
}
