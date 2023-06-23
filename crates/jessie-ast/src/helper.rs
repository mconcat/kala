use utils::{SharedString, OwnedString, OwnedSlice};

use crate::*;

// Expression

// Data Literal

fn data_literal(data_literal: DataLiteral) -> Expr {
    Expr::DataLiteral(Box::new(data_literal))
}

pub fn undefined() -> Expr {
    data_literal(DataLiteral::Undefined)
}

pub fn null() -> Expr {
    data_literal(DataLiteral::Null)
}

pub fn boolean(b: bool) -> Expr {
    data_literal(if b { DataLiteral::True } else { DataLiteral::False })
}

pub fn number(n: i64) -> Expr {
    data_literal(DataLiteral::Integer(OwnedString::from_string(n.to_string())))
}

pub fn string(s: &str) -> Expr {
    data_literal(DataLiteral::String(OwnedString::from_string(s.to_string())))
}

pub fn bigint(s: &str) -> Expr {
    data_literal(DataLiteral::Bigint(OwnedString::from_string(s.to_string())))
}

// Array

pub fn array(elements: Vec<Expr>) -> Expr {
    Expr::Array(Box::new(Array(OwnedSlice::from_vec(elements))))
}

// Record

pub fn record(fields: Vec<PropDef>) -> Expr {
    Expr::Record(Box::new(Record(OwnedSlice::from_vec(fields))))
}

pub fn keyvalue(key: &str, value: Expr) -> PropDef {
    PropDef::KeyValue(Box::new(Field::new_dynamic(SharedString::from_str(key))), value)
}
/* 
pub fn shorthand(key: &str, value: VariableCell) -> PropDef {
    PropDef::Shorthand(Box::new(Field::new_dynamic(SharedString::from_str(key))), value)
}
*/

// Functions

// BinaryExpr

fn binary_expr(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::BinaryExpr(Box::new(BinaryExpr(op, l, r)))
}

pub fn add(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Add, l, r)
}

// Variable

pub fn variable(name: &str) -> Expr {
    Expr::Variable(Box::new(VariableCell::new(SharedString::from_str(name))))
}