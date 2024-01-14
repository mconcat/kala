use std::cell::{OnceCell, Cell};
use std::rc::Rc;

use jessie_ast::Declaration;
use jessie_ast::module::{Module, ModuleItem, ExportClause, Script};
use kala_repr::function::Frame;
use kala_repr::{completion::Completion, slot::Slot};
use utils::map::Map;

use crate::expression::eval_expr;
use crate::interpreter::Interpreter;
use crate::statement::{eval_local_declaration, eval_statement};

pub fn eval_script(
    script: Script<Slot>,
) -> Completion {
    let mut interpreter = Interpreter::new(script.used_builtins, Frame::empty());

    let mut result = Slot::new_undefined();

    for statement in script.statements {
        result = eval_statement(&mut interpreter, &statement)?;
    }

    Completion::Value(result)
}

pub fn eval_module( 
    module: Module<Slot>,
) -> Completion {
    let mut export_default = OnceCell::new();

    let mut interpreter = Interpreter::new(module.used_builtins, Frame::empty());
    for item in module.body {
        match item {
            ModuleItem::ImportDeclaration(_) => unimplemented!("import"),
            ModuleItem::ModuleDeclaration(decl) => {
                let slot = eval_local_declaration(&mut interpreter, &decl.declaration)?;
                if decl.export_clause == ExportClause::ExportDefault {
                    export_default.set(slot);
                }
            }
        }
    }
    
    if let Some(default) = export_default.get() {
        Completion::Value(default.clone())
    } else {
        Completion::Normal
    }
}