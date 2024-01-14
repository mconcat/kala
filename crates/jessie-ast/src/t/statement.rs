use std::{rc::Rc, cell::RefCell};

use crate::{Expr, Statement, Pattern, Declaration, VariableDeclaration, IfStatement, Block, ElseArm, Function};

pub fn _const(pattern: impl Into<Pattern>, value: impl Into<Expr>) -> Statement {
    Statement::LocalDeclaration(Box::new(Declaration::Const(Box::new([VariableDeclaration{pattern: pattern.into(), value: Some(value.into())}]))))
}

pub fn _let(pattern: impl Into<Pattern>, value: Option<impl Into<Expr>>) -> Statement {
    Statement::LocalDeclaration(Box::new(Declaration::Let(Box::new([VariableDeclaration{pattern: pattern.into(), value: value.map(Into::into)}]))))
}

pub fn _decl_function(name: &str, scope: Option<crate::FunctionScope>, params: &[Pattern], body: impl Into<Block>) -> Statement {
    Statement::LocalDeclaration(Box::new(Declaration::Function(Rc::new(RefCell::new(Function{
        name: crate::FunctionName::Named(Rc::from(name)),
        parameters: params.into(),
        body: crate::ExprOrBlock::Block(body.into()),
        scope: scope.map(Into::into),
    })))))
}

pub fn _block(statements: &[Statement]) -> Statement {
    Statement::Block(Box::new(Block {
        statements: Box::from(statements),
        declarations: statements.iter().filter_map(|stmt| match stmt {
            Statement::LocalDeclaration(decl) => Some(*decl.clone()),
            _ => None,
        }).collect::<Vec<Declaration>>().into_boxed_slice(),
    }))
}


pub fn _if(cond: impl Into<Expr>, then: impl Into<Block>) -> Statement {
    Statement::IfStatement(Box::new(IfStatement{condition: cond.into(), consequent: then.into(), alternate: ElseArm::NoElse}))
}

pub fn _ifelse(cond: impl Into<Expr>, then: impl Into<Block>, else_: impl Into<Block>) -> Statement {
    Statement::IfStatement(Box::new(IfStatement{condition: cond.into(), consequent: then.into(), alternate: ElseArm::Else(else_.into())}))
}

pub fn _ifelse_if(cond: impl Into<Expr>, then: impl Into<Block>, else_if: impl Into<IfStatement>) -> Statement {
    Statement::IfStatement(Box::new(IfStatement{condition: cond.into(), consequent: then.into(), alternate: ElseArm::ElseIf(Box::new(else_if.into()))}))
}

pub fn _continue() -> Statement {
    Statement::Continue
}

pub fn _break() -> Statement {
    Statement::Break
}

pub fn _return_value(expr: impl Into<Expr>) -> Statement {
    Statement::Return(Box::new(expr.into()))
}

pub fn _return() -> Statement {
    Statement::ReturnEmpty
}

pub fn _throw(expr: impl Into<Expr>) -> Statement {
    Statement::Throw(Box::new(expr.into()))
}