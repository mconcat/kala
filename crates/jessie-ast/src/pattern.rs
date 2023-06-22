use utils::OwnedSlice;

use crate::{Expr, ExprDiscriminant, VariableCell, Record, PropDefDiscriminant, Field, AssignOp, LValue};

// Pattern is a subset of Expr
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>) = ExprDiscriminant::Spread as u8,
    Optional(Box<OptionalPattern>) = ExprDiscriminant::Assignment as u8,
    ArrayPattern(Box<ArrayPattern>) = ExprDiscriminant::Array as u8, // only Vec<Param> form is valid
    RecordPattern(Box<RecordPattern>) = ExprDiscriminant::Record as u8,
    Variable(Box<VariableCell>) = ExprDiscriminant::Variable as u8,
}

impl Pattern {
    pub fn optional(lvalue: VariableCell, expr: Expr) -> Self {
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
    Variable(Box<VariableCell>) = 12, // LValue::Variable
}

// ArrayPattern is a subset of Expr::Array
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPattern(pub OwnedSlice<Pattern>);

// RecordPattern is a subset of Expr::Record
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern(pub OwnedSlice<PropParam>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    KeyValue(Box<Field>, Pattern) = PropDefDiscriminant::KeyValue as u8,
    Shorthand(Box<Field>, VariableCell) = PropDefDiscriminant::Shorthand as u8,
    Rest(Pattern) = PropDefDiscriminant::Spread as u8,
}



/* 
impl Pattern {
    pub fn rest(pattern: &'a Self) -> Self {
        Pattern::Rest(pattern)
    }

    pub fn optional(name: &'a VariableCell, expr: &'a Expr) -> Self {
        Pattern::Optional(name, expr)
    }

    pub fn array(patterns: &'a [Self]) -> Self {
        Pattern::ArrayPattern(patterns)
    }

    pub fn record(props: &'a [PropParam]) -> Self {
        Pattern::RecordPattern(props)
    }

    pub fn variable(name: &'a VariableCell) -> Self {
        Pattern::Variable(name)
    }
}

impl From<Expr> for Pattern {
    fn from(value: Expr) -> Self {
        // Expression can be converted to pattern only if it is 
        // - a variable
        // - an assignment to a variable
        // - array compatible with destructuring
        // - object compatible with destructuring
        match value {
            Expr::Variable(name) => Pattern::Variable(name.into()),
            Expr::Assignment(assign) => unimplemented!("optional"),
            Expr::Array(arr) => unimplemented!("array"),
            Expr::Record(rec) => unimplemented!("record"), 
            _ => panic!("Cannot convert expr to pattern"),
        }
    }
}
*/