use std::{rc::Rc, cell::RefCell};

use jessie_ast::{Block, ExprOrBlock, Function, LValueOptional, OptionalPattern, Pattern, PropParam, Script, Variable, VariableDeclaration};

use crate::{state::ScopeState, scope_statement, scope_expression};

pub fn scope_script<T: Clone>(state: &mut ScopeState<T>, script: &mut Script) -> Result<(), &'static str> {
    state.enter_script()?;
    state.enter_block(&mut script.statements)?; 
    for stmt in script.statements.statements.iter_mut() {
        scope_statement(state, stmt)?;
    }
    //let declarations = state.exit_script();
    state.exit_block();
    state.exit_script()?;
    Ok(())
}


pub fn scope_variable<T: Clone>(state: &mut ScopeState<T>, var: &mut Variable) -> Result<(), &'static str> {
    state.use_variable(var)
}

pub fn scope_function_declaration<T: Clone>(state: &mut ScopeState<T>, func: Rc<RefCell<Function>>) -> Result<(), &'static str> {
    state.declare_function(func.clone())?;
    scope_function(state, &mut*func.borrow_mut())
}

pub fn scope_function<T: Clone>(state: &mut ScopeState<T>, func: &mut Function) -> Result<(), &'static str> {
    state.enter_function(func)?;

    match &mut func.body {
        ExprOrBlock::Expr(expr) => scope_expression(state, expr),
        ExprOrBlock::Block(block) => scope_block(state, block),
    }?;

    let scope = state.exit_function();

    func.scope = Some(Box::new(scope));

    Ok(())
}


pub fn scope_block<T: Clone>(state: &mut ScopeState<T>, block: &mut Block) -> Result<(), &'static str> {
    state.enter_block(block);
    println!("enter block {:?} /// {:?}", block.statements, block.declarations);
    for stmt in block.statements.iter_mut() {
        scope_statement(state, stmt)?;
    }
    state.exit_block();
    Ok(())
}