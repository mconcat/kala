use crate::{expression::Expr, DeclarationIndex};

// StatementItem in Jessie
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // The actual declaration is stored in the innermost function. DeclarationIndicies point to them.
    // When encountered, declaration statements initializes the variable to undefined, or with init value.
    LocalDeclaration(Box<Vec<usize>>),
    FunctionDeclaration(usize),
    Block(Box<Block>),
    IfStatement(Box<IfStatement>),
    // ForStatement(ForStatement),
    WhileStatement(Box<WhileStatement>),
    Continue,
    Break,
    Return(Box<Expr>),
    ReturnEmpty,
    Throw(Box<Expr>),
    // TryStatement(TryStatement),
    ExprStatement(Box<Expr>),
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Block(pub Vec<Statement>);

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Block(statements)
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