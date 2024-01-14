use std::{rc::Rc, cell::RefCell};

use jessie_ast::{Variable, VariableDeclaration, Function, Block, Pattern, PropParam, ExprOrBlock, OptionalPattern, LValueOptional};

use crate::{state::ScopeState, scope_statement, scope_expression};

pub fn scope_variable(state: &mut ScopeState, var: &mut Variable) -> Result<(), &'static str> {
    state.use_variable(var)
}

pub fn scope_function_declaration(state: &mut ScopeState, func: Rc<RefCell<Function>>) -> Result<(), &'static str> {
    state.declare_function(func.clone())?;
    scope_function(state, &mut*func.borrow_mut())
}

pub fn scope_function(state: &mut ScopeState, func: &mut Function) -> Result<(), &'static str> {
    state.enter_function(func)?;

    match &mut func.body {
        ExprOrBlock::Expr(expr) => scope_expression(state, expr),
        ExprOrBlock::Block(block) => scope_block(state, block),
    }?;

    let scope = state.exit_function();

    func.scope = Some(Box::new(scope));

    Ok(())
}


pub fn scope_block(state: &mut ScopeState, block: &mut Block) -> Result<(), &'static str> {
    state.enter_block(block);
    for stmt in block.statements.iter_mut() {
        scope_statement(state, stmt)?;
    }
    state.exit_block();
    Ok(())
}