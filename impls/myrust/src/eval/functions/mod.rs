mod native_function;

mod atom;
mod booleans;
mod control_flow;
mod debugging;
mod eval;
mod files;
mod lambdas;
mod lists;
mod math;
mod printing;
mod var_declarations;

use std::rc::Rc;

pub use native_function::*;

pub fn global_functions() -> Vec<Rc<dyn NativeFunction>> {
    let mut fns = vec![];
    fns.append(&mut control_flow::functions());
    fns.append(&mut math::functions());
    fns.append(&mut var_declarations::functions());
    fns.append(&mut lambdas::functions());
    fns.append(&mut printing::functions());
    fns.append(&mut lists::functions());
    fns.append(&mut booleans::functions());
    fns.append(&mut eval::functions());
    fns.append(&mut files::functions());
    fns.append(&mut debugging::functions());
    fns.append(&mut atom::functions());

    return fns;
}