use jessie_ast::*;

use crate::{state::ScopeState, scope_expression, scope_function_declaration, scope_block, scope_function};

pub fn scope_statement(state: &mut ScopeState, statement: &mut Statement) -> Result<(), &'static str> {
    match statement {
        Statement::LocalDeclaration(box decl) => match decl {
            Declaration::Function(func) => scope_function(state, &mut*func.borrow_mut()),
            _ => Ok(()),
        },
        Statement::Block(block) => scope_block(state, block),
        Statement::IfStatement(stmt) => scope_if(state, stmt),
        Statement::WhileStatement(stmt) => {
            // I believe that the scoping rule for loops are rather complex than this
            // TODO
            scope_expression(state, &mut stmt.condition)?;
            scope_block(state, &mut stmt.body)
        }
        Statement::Continue => Ok(()),
        Statement::Break => Ok(()),
        Statement::Throw(expr) => scope_expression(state, expr),
        Statement::Return(expr) => scope_expression(state, expr),
        Statement::ReturnEmpty => Ok(()),
        Statement::ExprStatement(expr) => scope_expression(state, expr),
    }
}

fn scope_if(state: &mut ScopeState, stmt: &mut IfStatement) -> Result<(), &'static str> {
    scope_expression(state, &mut stmt.condition)?;
    scope_block(state, &mut stmt.consequent)?;
    match &mut stmt.alternate {
        ElseArm::NoElse => Ok(()),
        ElseArm::Else(block) => scope_block(state, block),
        ElseArm::ElseIf(elseif) => scope_if(state, elseif),
    }
}
