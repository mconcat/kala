use std::rc::Rc;

use crate::{expression::Expr, DeclarationIndex, VariableDeclaration, FunctionDeclaration};

#[repr(u8)]
pub enum StatementDiscriminant {
    LocalDeclaration = 0,
    FunctionDeclaration = 1,
    Block = 2,
    IfStatement = 3,
    // ForStatement = 4
    WhileStatement = 5,
    Continue = 6,
    Break = 7,
    Return = 8,
    ReturnEmpty = 9,
    Throw = 10,
    // TryStatement = 11,
    ExprStatement = 12,
}

#[repr(u8)]
// StatementItem in Jessie
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // The actual declaration is stored in the innermost function. DeclarationIndicies point to them.
    // When encountered, declaration statements initializes the variable to undefined, or with init value.
    // TODO: we can actually remove the declaration indicies as they always match with the order they appears inside the function, but I will just use u32 indices for now - refactor later
    VariableDeclaration(Box<Vec<(u32, Rc<VariableDeclaration>)>>) = StatementDiscriminant::LocalDeclaration as u8,
    FunctionDeclaration(u32, Rc<FunctionDeclaration>) = StatementDiscriminant::FunctionDeclaration as u8,
    Block(Box<Block>) = StatementDiscriminant::Block as u8,
    IfStatement(Box<IfStatement>) = StatementDiscriminant::IfStatement as u8,
    // ForStatement(ForStatement),
    WhileStatement(Box<WhileStatement>) = StatementDiscriminant::WhileStatement as u8,
    Continue = StatementDiscriminant::Continue as u8,
    Break = StatementDiscriminant::Break as u8,
    Return(Box<Expr>) = StatementDiscriminant::Return as u8,
    ReturnEmpty = StatementDiscriminant::ReturnEmpty as u8,
    Throw(Box<Expr>) = StatementDiscriminant::Throw as u8,
    // TryStatement(TryStatement),
    ExprStatement(Box<Expr>) = StatementDiscriminant::ExprStatement as u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Block{statements}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Expr,
    pub consequent: Block,
    pub alternate: ElseArm,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElseArm {
    NoElse,
    Else(Block),
    ElseIf(Box<IfStatement>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}