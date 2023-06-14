use crate::{operation::*, Function, Record, Assignment, VariableCell};

// paren, function, literal, array, record, variable

// PrimaryExpr, Operator Expressions(CondExpr, BinaryExpr, UnaryExpr, CallExpr), AssignExpr
// are all collapsed into single Expr type.
// Be sure not to represent any invalid states.
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Expr {
    DataLiteral(Box<DataLiteral>) = 0,
    Array(Box<Array>) = 1,
    Record(Box<Record>) = 2,
    ArrowFunc(Box<Function>) = 3,
    FunctionExpr(Box<Function>) = 4,
    Assignment(Box<Assignment>) = 5,
    CondExpr(Box<CondExpr>) = 6,
    BinaryExpr(Box<BinaryExpr>) = 7,
    UnaryExpr(Box<UnaryExpr>) = 8,
    CallExpr(Box<CallExpr>) = 9,
    // QuasiExpr() = 10
    ParenedExpr(Box<Expr>) = 11,
    Variable(Box<VariableCell>) = 12,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array(pub Vec<Element>);


#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Expr(Expr),
    Spread(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataLiteral {
    Null,
    False,
    True,
    Integer(String),
    Decimal(String),
    String(String),
    Undefined,
    Bigint(String),
}



/*
impl Expr {
    pub fn new_number(n: i64) -> Self {
        Expr::DataLiteral(DataLiteral::Integer(n.to_string()))
    }

    pub fn new_add(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Add, l, r)))
    }

    pub fn new_sub(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Sub, l, r)))
    }
}

*/


/*
#[derive(Debug, PartialEq, Clone)]
pub enum BlockOrExpr {
    Block(Vec<Statement>),
    Expr(Expr), // only appears for arrow functions
}
*/

/* 
// Function is used for function declaration, function expressions, and arrow functions.
#[derive(Debug, PartialEq, Clone)]
pub struct Function{
    pub name: Option<DefVariable>,


    pub parameters: DeclarationPointer, // must be Parameters, TODO
    pub typeann: Option<TypeAnn>,
    
    // block body
    pub statements: Vec<Statement>,

    // arrow function expression body
    pub expression: Option<Expr>,

    pub scope: Scope,

    pub captures: Option<Vec<(Field, MutableDeclarationPointer)>>, 
}

impl Function {
    pub fn from_body(name: Option<DefVariable>, parameters: DeclarationPointer, typeann: Option<TypeAnn>, block_or_expr: BlockOrExpr, scope: Scope) -> Self {
        match block_or_expr {
            BlockOrExpr::Block(statements) => Function {
                name,
                parameters,
                typeann,
                statements,
                expression: None,
                scope,
            },
            BlockOrExpr::Expr(expression) => Function {
                name,
                parameters,
                typeann,
                statements: vec![],
                expression: Some(expression),
                scope,
            },
        }
    }
}

*/

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr(pub BinaryOp, pub Expr, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub struct CondExpr(pub Expr, pub Expr, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Vec<UnaryOp>,
    pub expr: Expr,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum CallPostOp {
    Index(Expr) = 0,
    Member(String) = 1,
    // QuasiExpr = 2
    Call(Vec<Arg>) = 3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub expr: Expr,
    pub post_ops: Vec<CallPostOp>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Expr(Expr),
    Spread(Expr),
}
