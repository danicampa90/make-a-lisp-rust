use std::{fmt::Display, rc::Rc};

use crate::{functions::NativeFunction, read::AstNode};

use super::EnvironmentEntryValue;

#[derive(PartialEq)]
pub struct EnvironmentEntry {
    name: String,
    value: EnvironmentEntryValue,
}
impl EnvironmentEntry {
    pub fn to_ast_node(self: Rc<Self>) -> AstNode {
        match &self.value {
            EnvironmentEntryValue::Value(node) => node.clone(),
            EnvironmentEntryValue::NativeFunction(_) => AstNode::FunctionPtr(self.clone()),
        }
    }
    pub fn value(&self) -> &EnvironmentEntryValue {
        &self.value
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn new_native(func: Rc<dyn NativeFunction>) -> Self {
        Self {
            name: func.name(),
            value: EnvironmentEntryValue::NativeFunction(func),
        }
    }
    pub fn new_ast_value(name: String, val: AstNode) -> Self {
        Self {
            name,
            value: EnvironmentEntryValue::Value(val),
        }
    }
}

impl Display for EnvironmentEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            EnvironmentEntryValue::Value(ast) => write!(f, "<variable {} = {:?}>", self.name, ast),
            EnvironmentEntryValue::NativeFunction(_) => write!(f, "<function {}>", self.name),
        }
    }
}
