use crate::{operation::*, Function, Record, Assignment, VariableCell, VariablePointer, Field};
use utils::{SharedString, OwnedSlice, OwnedString};

// paren, function, literal, array, record, variable

#[repr(u8)]
pub enum ExprDiscriminant {
    DataLiteral = 0,
    Array = 1,
    Record = 2,
    ArrowFunc = 3,
    FunctionExpr = 4,
    Assignment = 5,
    CondExpr = 6,
    BinaryExpr = 7,
    UnaryExpr = 8,
    CallExpr = 9,
    // QuasiExpr() = 10
    ParenedExpr = 11,
    Variable = 12,
    Spread = 13,
}

// PrimaryExpr, Operator Expressions(CondExpr, BinaryExpr, UnaryExpr, CallExpr), AssignExpr
// are all collapsed into single Expr type.
// Be sure not to represent any invalid states.
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Expr {
    DataLiteral(Box<DataLiteral>) = ExprDiscriminant::DataLiteral as u8,
    Array(Box<Array>) = ExprDiscriminant::Array as u8,
    Record(Box<Record>) = ExprDiscriminant::Record as u8,
    ArrowFunc(Box<Function>) = ExprDiscriminant::ArrowFunc as u8,
    FunctionExpr(Box<Function>) = ExprDiscriminant::FunctionExpr as u8,
    Assignment(Box<Assignment>) = ExprDiscriminant::Assignment as u8,
    CondExpr(Box<CondExpr>) = ExprDiscriminant::CondExpr as u8,
    BinaryExpr(Box<BinaryExpr>) = ExprDiscriminant::BinaryExpr as u8,
    UnaryExpr(Box<UnaryExpr>) = ExprDiscriminant::UnaryExpr as u8,
    CallExpr(Box<CallExpr>) = ExprDiscriminant::CallExpr as u8,
    // QuasiExpr() = 10
    ParenedExpr(Box<Expr>) = ExprDiscriminant::ParenedExpr as u8,
    Variable(Box<VariableCell>) = ExprDiscriminant::Variable as u8,
    Spread(Box<Expr>) = ExprDiscriminant::Spread as u8, // for array elements
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Array(pub OwnedSlice<Expr>);

#[derive(Debug, PartialEq, Clone)]
pub struct KeyValue {
    pub key: Field,
    pub value: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataLiteral {
    Null,
    False,
    True,
    Integer(OwnedString),
    Decimal(OwnedString),
    String(OwnedString),
    Undefined,
    Bigint(OwnedString),
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
    Member(SharedString) = 1,
    // QuasiExpr = 2
    Call(OwnedSlice<Expr>) = 3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub expr: Expr,
    pub post_ops: OwnedSlice<CallPostOp>,
}