use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::{Environment, EnvironmentEntry};

#[derive(Clone)]
pub struct SharedEnvironment(pub Rc<RefCell<Environment>>);

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

    for func in crate::functions::global_functions() {
        env.set_owned(EnvironmentEntry::new_native(func));
    }

    let global = Environment::new_child(env.as_shared());
    global.as_shared()
}
