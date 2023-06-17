use std::{cell::RefCell, rc::Rc};

use crate::read::AstNode;

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(AtomFn),
        Rc::new(IsAtomFn),
        Rc::new(DerefFn),
        Rc::new(ResetBangFn),
        Rc::new(SwapBangFn),
    ]
}

struct AtomFn;
impl NativeFunction for AtomFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "atom".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        Ok(FunctionCallResultSuccess::Value(AstNode::Atom(Rc::new(
            RefCell::new(data.destructure().0.remove(0)),
        ))))
    }
}

struct IsAtomFn;
impl NativeFunction for IsAtomFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "atom?".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        Ok(FunctionCallResultSuccess::Value(AstNode::Bool(
            data.destructure().0.remove(0).try_unwrap_atom().is_ok(),
        )))
    }
}

struct DerefFn;
impl NativeFunction for DerefFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "deref".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(1), Some(1))?;
        Ok(FunctionCallResultSuccess::Value(
            data.destructure()
                .0
                .remove(0)
                .try_unwrap_atom()?
                .borrow()
                .clone(),
        ))
    }
}

struct ResetBangFn;
impl NativeFunction for ResetBangFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "reset!".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), Some(2))?;
        let mut ast = data.destructure().0;
        let atom = ast.remove(0).try_unwrap_atom()?;
        let value = ast.remove(0);

        atom.replace(value.clone());

        Ok(FunctionCallResultSuccess::Value(value))
    }
}
struct SwapBangFn;
impl NativeFunction for SwapBangFn {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "swap!".to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        data.check_parameters_count_range(Some(2), None)?;
        let evaluator = data.evaluator();
        let (mut ast, env) = data.destructure();

        let atom: Rc<RefCell<AstNode>> = ast.remove(0).try_unwrap_atom()?;
        let function = ast.remove(0);

        let atom_value: AstNode = atom.borrow().clone();
        ast.insert(0, atom_value);
        ast.insert(0, function);

        let new_value = evaluator.eval(AstNode::List(ast), env)?; // (<function> <atom_value> <rest...>)
        atom.replace(new_value.clone());

        Ok(FunctionCallResultSuccess::Value(new_value))
    }
}
