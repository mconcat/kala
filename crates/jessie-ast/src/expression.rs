use crate::{operation::*, Function, Record, Assignment, VariableCell, VariablePointer, Field, Pattern, PropDef};
use utils::{SharedString, FxMap, Map};
use sha3::{Digest, Sha3_256};

// paren, function, literal, array, record, variable

#[repr(u8)]
pub enum ExprDiscriminant {
    DataLiteral = 0,
    Array = 1,
    Record = 2,
    Function = 3,
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
    Function(Box<Function>) = ExprDiscriminant::Function as u8,
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
pub struct Array(pub Vec<Expr>);

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
    Integer(u64),
    Decimal([u64;2]),
    String(SharedString),
    Undefined,
    Bigint(bool, Box<[u64]>),
}

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
    Call(Vec<Expr>) = 3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub expr: Expr,
    pub post_ops: Vec<CallPostOp>,
}
