use std::{rc::{self, Rc}};

use jessie_ast::{Statement, IfStatement, ElseArm, WhileStatement, Block, Expr, VariableIndex, Declaration, VariableDeclaration, Pattern, Function};

use crate::{expression::eval_expr, interpreter::Interpreter};

use kala_repr::{completion::Completion, slot::Slot};

pub fn eval_statement(interpreter: &mut Interpreter, statement: &Statement) -> Completion {
    match statement {
        Statement::LocalDeclaration(local) => eval_local_declaration(interpreter, local),
        Statement::Block(block) => eval_block(interpreter, &block),
        Statement::IfStatement(if_statement) => eval_if(interpreter, &if_statement),
        Statement::WhileStatement(while_statement) => eval_while(interpreter, &while_statement),
        Statement::Continue => Completion::Continue,
        Statement::Break => Completion::Break,
        Statement::Return(expr) => Completion::Return(eval_expr(interpreter, &*expr)?),
        Statement::ReturnEmpty => Completion::ReturnEmpty,
        Statement::Throw(expr) => Completion::Throw(eval_expr(interpreter, &*expr)?),
        Statement::ExprStatement(expr) => eval_expr(interpreter, &expr).into(),
    }
}

pub fn eval_local_declaration(interpreter: &mut Interpreter, decl: &Declaration) -> Completion {
    match decl {
        Declaration::Const(local) => {
            for item in &**local {
                eval_const(interpreter, item)?;
            }
        }
        Declaration::Let(local) => {
            for item in &**local {
                eval_let(interpreter, item)?;
            }
        }
        Declaration::Function(func) => {
            eval_function_declaration(interpreter, &func)?; 
        },
    }

    Completion::Normal
}

pub fn initialize_pattern(interpreter: &mut Interpreter, pattern: &Pattern, initializer: Slot) -> Completion {
    match pattern {
        Pattern::Variable(var) => {
            let variable = interpreter.fetch_variable(var.index.get())?;
            /* 
            if variable.is_const() {
                return Completion::Throw(interpreter.create_type_error("Cannot assign to const variable"));
            }
            */
            *variable = initializer;
            Completion::Normal
        }
        _ => unimplemented!("initialize_pattern"),
    }
}

pub fn eval_const(interpreter: &mut Interpreter, decl: &VariableDeclaration) -> Completion {
    let initializer = &decl.value;
    if initializer.is_none() {
        return Completion::Normal; // TODO
    } 
    let initializer = eval_expr(interpreter, initializer.as_ref().unwrap())?;
    
    initialize_pattern(interpreter, &decl.pattern, initializer)
}

pub fn eval_let(interpreter: &mut Interpreter, decl: &VariableDeclaration) -> Completion {
    let initializer = &decl.value;
    if initializer.is_none() {
        return Completion::Normal; // TODO
    } 
    let initializer = eval_expr(interpreter, initializer.as_ref().unwrap())?;
    
    initialize_pattern(interpreter, &decl.pattern, initializer)
}

pub fn eval_function_declaration(interpreter: &mut Interpreter, func: &Function) -> Completion {
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
    let condition = eval_expr(interpreter, &statement.condition)?;

    if condition.is_truthy() {
        eval_block(interpreter, &statement.consequent)
    } else {
        match &statement.alternate {
            ElseArm::NoElse => Completion::Normal,
            ElseArm::Else(block) => eval_block(interpreter, &block),
            ElseArm::ElseIf(if_statement) => eval_if(interpreter, &*if_statement),
        }
    }
}

pub fn eval_while(interpreter: &mut Interpreter, statement: &WhileStatement) -> Completion {
    while {
        let condition = eval_expr(interpreter, &statement.condition)?;
        condition.is_truthy()   
    } {
        // TODO: match completion, right now it breaks on any completion(including continue)
        eval_block(interpreter, &statement.body)?;
    }

    Completion::Normal
}

pub fn eval_throw(interpreter: &mut Interpreter, expr: &Expr) -> Completion {
    let exception = eval_expr(interpreter, &expr)?;
    Completion::Throw(exception)
}