use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::EvalError;
use crate::read::AstNode;

type NativeFunction = fn(Vec<AstNode>, &SharedEnvironment) -> Result<AstNode, EvalError>;
pub enum EnvironmentEntryValue {
    AstNode(AstNode),
    NativeFunction(NativeFunction),
}

pub struct EnvironmentEntry {
    name: String,
    eval_parameters: bool,
    value: EnvironmentEntryValue,
}
impl EnvironmentEntry {
    pub fn to_ast_node(self: Rc<Self>) -> AstNode {
        AstNode::FunctionPtr(self.clone())
    }
    pub fn value(&self) -> &EnvironmentEntryValue {
        &self.value
    }
    pub fn eval_parameters(&self) -> bool {
        self.eval_parameters
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn new_native_function(name: String, func: NativeFunction) -> Self {
        Self {
            name,
            eval_parameters: true,
            value: EnvironmentEntryValue::NativeFunction(func),
        }
    }
    pub fn new_special_atom(name: String, func: NativeFunction) -> Self {
        Self {
            name,
            eval_parameters: false,
            value: EnvironmentEntryValue::NativeFunction(func),
        }
    }
}
impl Display for EnvironmentEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(fn_ptr {} - eval_params:{})",
            self.name, self.eval_parameters
        )
    }
}

pub struct Environment {
    shared_definitions: HashMap<String, Rc<EnvironmentEntry>>,
    parent: Option<SharedEnvironment>,
}

impl Environment {
    pub fn lookup(&self, name: &String) -> Option<Rc<EnvironmentEntry>> {
        match self.shared_definitions.get(name).cloned() {
            Some(val) => Some(val),
            None => match &self.parent {
                Some(parent) => parent.clone().borrow().lookup(name),
                None => None,
            },
        }
    }
    pub fn new_root() -> Environment {
        Self {
            shared_definitions: HashMap::new(),
            parent: None,
        }
    }
    pub fn new_child(parent: SharedEnvironment) -> Environment {
        Self {
            shared_definitions: HashMap::new(),
            parent: Some(parent),
        }
    }
    pub fn as_shared(self) -> SharedEnvironment {
        Rc::new(RefCell::new(self))
    }
    pub fn add_entry(&mut self, entry: Rc<EnvironmentEntry>) {
        self.shared_definitions.insert(entry.name().clone(), entry);
    }
    pub fn add_entry_owned(&mut self, entry: EnvironmentEntry) {
        self.add_entry(Rc::new(entry))
    }
}

pub type SharedEnvironment = Rc<RefCell<Environment>>;
