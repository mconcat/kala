use crate::Expr;
use crate::VariableCell;
use std::mem;


#[derive(Debug, PartialEq, Clone)]
pub struct Assignment<'a>(pub AssignOp, pub LValue<'a>, pub Expr<'a>);

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
pub enum LValue<'a> {
    CallLValue(&'a CallLValue<'a>) = 9,
    Variable(&'a VariableCell<'a>) = 12,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValueCallPostOp<'a> {
    Index(Expr<'a>) = 0,
    Member(&'a str) = 1,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallLValue<'a> {
    pub expr: Expr<'a>,
    pub post_op: &'a [LValueCallPostOp<'a>],
}

impl<'a> From<LValue<'a>> for Expr<'a> {
    fn from(lv: LValue) -> Self {
        // Super unsafe, add bunch of test cases later
        unsafe { mem::transmute(lv) }
    }
}

impl<'a> From<Expr<'a>> for LValue<'a> {
    fn from(value: Expr) -> Self {
         // must be called only when the expr is transmutable to LValue
         // Super super unsafe
        unsafe { mem::transmute(value) }
    }
}