use crate::{expression::Expr, VariableCell};

// StatementItem in Jessie
#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Declaration(&'a VariableCell<'a>),
    Block(&'a Block<'a>),
    IfStatement(&'a IfStatement<'a>),
    // ForStatement(ForStatement),
    WhileStatement(&'a WhileStatement<'a>),
    Continue,
    Break,
    Return(&'a Expr<'a>),
    ReturnEmpty,
    Throw(&'a Expr<'a>),
    // TryStatement(TryStatement),
    ExprStatement(&'a Expr<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block<'a> {
    pub statements: &'a [Statement<'a>],
}

impl<'a> Block<'a> {
    pub fn new(statements: &'a [Statement<'a>]) -> Self {
        Block {
            statements,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement<'a> {
    pub arms: &'a [IfArm<'a>],
    pub else_arm: Option<Block<'a>>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct IfArm<'a> {
    pub condition: Expr<'a>,
    pub consequent: Block<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement<'a> {
    pub condition: Expr<'a>,
    pub body: Block<'a>,
}