use std::rc::Rc;

use crate::{functions::NativeFunction, read::AstNode};

pub enum EnvironmentEntryValue {
    Value(AstNode),
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
