use jessie_ast::{DeclarationIndex, Statement, IfStatement, ElseArm, WhileStatement, Block, LocalDeclaration};

use crate::{expression::{Interpreter, eval_expr}, interpreter::{Completion}};

pub fn eval_statement(interpreter: &mut Interpreter, statement: Statement) -> Completion {
    match statement {
        Statement::LocalDeclaration(local) => eval_local_declaration(interpreter, local),
        Statement::FunctionDeclaration(func) => eval_function_declaration(interpreter, func),
        Statement::Block(block) => eval_block(interpreter, &block),
        Statement::IfStatement(if_statement) => eval_if(interpreter, &if_statement),
        Statement::WhileStatement(while_statement) => eval_while(interpreter, &while_statement),
        Statement::Continue => Completion::Continue,
        Statement::Break => Completion::Break,
        Statement::Return(expr) => Completion::Return(eval_expr(interpreter, &*expr)?),
        Statement::ReturnEmpty => Completion::ReturnEmpty,
        Statement::Throw(expr) => Completion::Throw(eval_expr(interpreter, &*expr)?),
        Statement::ExprStatement(expr) => eval_expr(interpreter, &expr),
    }
}

pub fn eval_local_declaration(interpreter: &mut Interpreter, local: Vec<u32>) -> Completion {
    for declaration_index in local {
        interpreter.initialize_local(declaration_index)?;
    }

    Completion::Normal
}

pub fn eval_function_declaration(interpreter: &mut Interpreter, func: u32) -> Completion {
    // Functions are hoisted, so they are already implicitly initialized.
    // TODO: unreachable?

    Completion::Normal
}

pub fn eval_block(interpreter: &mut Interpreter, block: &Block) -> Completion {
    for statement in &block.statements {
        eval_statement(interpreter, statement)?;
    }

    Completion::Normal
}

pub fn eval_if(interpreter: &mut Interpreter, statement: &IfStatement) -> Completion {
    if eval_expr(interpreter, &statement.condition)?.is_truthy() {
        eval_block(interpreter, &statement.consequent)
    } else {
        match statement.alternate {
            ElseArm::NoElse => Completion::Normal,
            ElseArm::Else(block) => eval_block(interpreter, &block),
            ElseArm::ElseIf(if_statement) => eval_if(interpreter, &*if_statement),
        }
    }
}

pub fn eval_while(interpreter: &mut Interpreter, statement: &WhileStatement) -> Completion {
    while eval_expr(interpreter, &statement.condition)?.is_truthy() {
        eval_block(interpreter, &statement.body)?;
    }

    Completion::Normal
}

pub fn eval_throw(interpreter: &mut Interpreter, expr: &Expr) -> Completion {
    let exception = eval_expr(interpreter, &expr)?;
    Completion::Throw(exception)
}