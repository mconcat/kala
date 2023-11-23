use utils::{FxMap, Map, SharedString};

use crate::{Expr, ExprDiscriminant, Record, Field, AssignOp, LValue, Variable};

// Pattern is a subset of Expr
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>) = ExprDiscriminant::Spread as u8,
    Optional(Box<OptionalPattern>) = ExprDiscriminant::Assignment as u8,
    ArrayPattern(Box<ArrayPattern>) = ExprDiscriminant::Array as u8, // only Vec<Param> form is valid
    RecordPattern(Box<RecordPattern>) = ExprDiscriminant::Record as u8,
    Variable(Box<Variable>) = ExprDiscriminant::Variable as u8,
}

impl Pattern {
    pub fn optional(lvalue: Variable, expr: Expr) -> Self {
        Pattern::Optional(Box::new(OptionalPattern(OptionalOp::Optional, LValueOptional::Variable(Box::new(lvalue)), expr)))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct OptionalPattern(pub OptionalOp, pub LValueOptional, pub Expr);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum OptionalOp {
    Optional = AssignOp::Assign as u8,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValueOptional {
    Variable(Box<Variable>) = 12, // LValue::Variable
}

// ArrayPattern is a subset of Expr::Array
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPattern(pub Vec<Pattern>);

// RecordPattern is a subset of Expr::Record
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern(pub Vec<PropParam>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    KeyValue(Box<Field>, Pattern),
    Shorthand(Box<Field>, Box<Variable>),
    Rest(Box<Variable>),
}