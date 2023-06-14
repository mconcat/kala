use crate::{Expr, VariableCell};
use std::mem;


#[derive(Debug, PartialEq, Clone)]
pub struct Assignment(pub AssignOp, pub LValue, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub enum AssignOp {
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignExp,
    AssignLShift,
    AssignRShift,
    AssignURShift,
    AssignBitAnd,
    AssignBitXor,
    AssignBitOr,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    CallLValue(Box<CallLValue>) = 9,
    Variable(Box<VariableCell>) = 12,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValueCallPostOp {
    Index(Expr) = 0,
    Member(String) = 1,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallLValue {
    pub expr: Expr,
    pub post_op: Vec<LValueCallPostOp>,
}

impl From<LValue> for Expr {
    fn from(lv: LValue) -> Self {
        // Super unsafe, add bunch of test cases later
        unsafe { mem::transmute(lv) }
    }
}

impl From<Expr> for LValue {
    fn from(value: Expr) -> Self {
         // must be called only when the expr is transmutable to LValue
         // Super super unsafe
        unsafe { mem::transmute(value) }
    }
}