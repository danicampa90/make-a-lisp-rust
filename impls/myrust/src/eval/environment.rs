use std::{cell::RefCell, collections::HashMap, fmt::Display, ops::Deref, rc::Rc};

use super::NativeFunction;
use crate::read::{AstNode, AstNodeRef};

pub enum EnvironmentEntryValue {
    Value(AstNodeRef),
    NativeFunction(Rc<dyn NativeFunction>),
}

impl PartialEq for EnvironmentEntryValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::NativeFunction(l0), Self::NativeFunction(r0)) => {
                Rc::as_ptr(l0) == Rc::as_ptr(r0)
            }
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub struct EnvironmentEntry {
    name: String,
    value: EnvironmentEntryValue,
}
impl EnvironmentEntry {
    pub fn to_ast_node(self: Rc<Self>) -> AstNodeRef {
        match &self.value {
            EnvironmentEntryValue::Value(node) => node.clone(),
            EnvironmentEntryValue::NativeFunction(_) => AstNode::FunctionPtr(self.clone()).into(),
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

pub struct Environment {
    shared_definitions: HashMap<String, Rc<EnvironmentEntry>>,
    parent: Option<SharedEnvironment>,
}

impl Environment {
    pub fn find(&self, name: &String) -> Option<Rc<EnvironmentEntry>> {
        match self.shared_definitions.get(name).cloned() {
            Some(val) => Some(val),
            None => match &self.parent {
                Some(parent) => parent.clone().borrow().find(name),
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
    pub fn parent(&self) -> Option<&SharedEnvironment> {
        self.parent.as_ref()
    }
    pub fn new_child(parent: SharedEnvironment) -> Environment {
        Self {
            shared_definitions: HashMap::new(),
            parent: Some(parent),
        }
    }
    pub fn as_shared(self) -> SharedEnvironment {
        SharedEnvironment(Rc::new(RefCell::new(self)))
    }
    pub fn set(&mut self, entry: Rc<EnvironmentEntry>) {
        self.shared_definitions.insert(entry.name().clone(), entry);
    }
    pub fn set_owned(&mut self, entry: EnvironmentEntry) {
        self.set(Rc::new(entry))
    }
}

#[derive(Clone)]
pub struct SharedEnvironment(Rc<RefCell<Environment>>);

impl Deref for SharedEnvironment {
    type Target = Rc<RefCell<Environment>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// comparison of environments is a pointers comparison
impl PartialEq for SharedEnvironment {
    fn eq(&self, other: &Self) -> bool {
        other.as_ptr() == self.as_ptr()
    }
}

impl SharedEnvironment {
    pub fn get_root(&self) -> SharedEnvironment {
        let self_ref = self.0.borrow();
        let parent = self_ref.parent();
        match parent {
            None => return self.clone(),
            Some(parent) => parent.get_root(),
        }
    }
}

pub fn new_base_environment() -> SharedEnvironment {
    let mut env = Environment::new_root();

    for func in super::functions::global_functions() {
        env.set_owned(EnvironmentEntry::new_native(func));
    }

    let global = Environment::new_child(env.as_shared());
    global.as_shared()
}
