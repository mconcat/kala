// Local row type inference without subtype relation
// We assume that subtype usage mostly happens in between different function calls.

use jessie_ast::{Function, Statement};

pub struct InferenceState {

}

pub fn infer_top_level_function(state: &mut InferenceState, function: &Function) {
    for statement in function.statements {
        infer_statement(state, statement);
    }
}

pub fn infer_statement(state: &mut InferenceState, statement: &Statement) {
    match statement {
        Statement::LocalDeclaration(declarations) => {
            for declaration in declarations {
                infer_declaration(state, declaration);
            }
        },
        Statement::FunctionDeclaration(declaration) => {
            infer_declaration(state, declaration);
        },
        Statement::Block(block) => {
            for statement in &block.0 {
                infer_statement(state, statement);
            }
        },
        Statement::IfStatement(if_statement) => {
            infer_expression(state, &if_statement.condition);
            infer_statement(state, &if_statement.consequent);
            match &if_statement.alternate {
                ElseArm::NoElse => {},
                ElseArm::Else(block) => {
                    infer_statement(state, block);
                },
                ElseArm::ElseIf(else_if) => {
                    infer_statement(state, else_if);
                },
            }
        },
        Statement::WhileStatement(while_statement) => {
            infer_expression(state, &while_statement.condition);
            infer_statement(state, &while_statement.body);
        },
        Statement::Continue => {},
        Statement::Break => {},
        Statement::Return(expr) => {
            infer_expression(state, expr);
        },
        Statement::ReturnEmpty => {},
        Statement::Throw(expr) => {
            infer_expression(state, expr);
        },
        Statement::ExprStatement(expr) => {
            infer_expression(state, expr);
        },
    }
}