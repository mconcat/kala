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

impl UnsafeInto<Expr> for Pattern {
    unsafe fn unsafe_into(self) -> Expr {
        std::mem::transmute(self)
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
pub struct ArrayPattern(pub Vec<Pattern>);

// RecordPattern is a subset of Expr::Record
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern(pub Vec<PropParam>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    KeyValue(Box<Field>, Pattern) = PropDefDiscriminant::KeyValue as u8,
    Shorthand(Box<Field>, VariableCell) = PropDefDiscriminant::Shorthand as u8,
    Rest(Pattern) = PropDefDiscriminant::Spread as u8,
}
