use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{EnvironmentEntry, SharedEnvironment};

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
