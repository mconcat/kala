use std::cell::{OnceCell, Cell};
use std::rc::Rc;

use jessie_ast::LocalDeclaration;
use jessie_ast::module::ModuleBody;
use kala_repr::function::Frame;
use kala_repr::{completion::Completion, slot::Slot};
use utils::{SharedString, FxMap};
use utils::map::Map;

use crate::expression::eval_expr;
use crate::interpreter::Interpreter;
use crate::statement::{eval_local_declaration, eval_function_declaration};

pub fn eval_declaration(interpreter: &mut Interpreter, index: u32, declaration: &Rc<LocalDeclaration>) -> Completion {
    match declaration.as_ref() {
        LocalDeclaration::Const { .. } => eval_local_declaration(interpreter, &Box::new(vec![(index, declaration.clone())])),
        LocalDeclaration::Let { .. } => eval_local_declaration(interpreter, &Box::new(vec![(index, declaration.clone())])),
        LocalDeclaration::Function { .. } => eval_function_declaration(interpreter, declaration),
    }
}

pub fn eval_module(
    mut builtins_map: FxMap<Slot>,
    module: ModuleBody
) -> Completion {
    let builtins = module.builtins.iter().map(|name| Cell::new(builtins_map.get(name.clone()).unwrap().clone())).collect::<Vec<Cell<Slot>>>();

    let frame = Frame::empty();

    let mut interpreter = Interpreter::new(Rc::new(OnceCell::from(builtins)), frame);

    let mut export_default = OnceCell::new();

    let mut results = Vec::with_capacity(module.globals.len());


    for (i, (export_clause, variable_declaration)) in module.globals.iter().enumerate() {
        if export_clause.is_default() {
            if export_default.set(i).is_err() {
                panic!("Multiple default exports");
            }
        }

        results.push(eval_declaration(&mut interpreter, i.try_into().unwrap(), variable_declaration)?);
    };

    if export_default.get().is_some() {
        return Completion::Value(results[*export_default.get().unwrap()].clone());
    }

    Completion::Value(results.last().unwrap().clone())
}