use std::{rc::Rc, cell::OnceCell};

use crate::{Expr, DataLiteral, Array, PropDef, Record, BinaryExpr, UnaryOp, UnaryExpr, CallPostOp, Assignment, LValue, Variable, Pattern, Block, Function};

pub fn _null() -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::Null))
}

pub fn _undefined() -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::Undefined))
}

pub fn _number(n: i64) -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::Integer(n)))
}

pub fn _string(s: &str) -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::String(Rc::from(s))))
}

pub fn _false() -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::False))
}

pub fn _true() -> Expr {
    Expr::DataLiteral(Box::new(DataLiteral::True))
}

pub fn _array(elements: &[Expr]) -> Expr {
    Expr::Array(Box::new(Array(Box::from(elements))))
}

pub fn _record(props: &[PropDef]) -> Expr {
    Expr::Record(Box::new(Record(Box::from(props))))
}

fn _binary_expr(op: crate::BinaryOp, x: impl Into<Expr>, y: impl Into<Expr>) -> Expr {
    Expr::BinaryExpr(Box::new(BinaryExpr(op, x.into(), y.into())))
}

pub fn _add(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Add, left, right)
}

pub fn _sub(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Sub, left, right)
}

pub fn _mul(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Mul, left, right)
}

pub fn _div(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Div, left, right)
}

pub fn _mod(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Mod, left, right)
}

pub fn _eq(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::StrictEqual, left, right)
}

pub fn _ne(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::StrictNotEqual, left, right)
}

pub fn _lt(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::LessThan, left, right)
}

pub fn _le(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::LessThanEqual, left, right)
}

pub fn _gt(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::GreaterThan, left, right)
}

pub fn _ge(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::GreaterThanEqual, left, right)
}

pub fn _and(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::And, left, right)
}

pub fn _or(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
    _binary_expr(crate::BinaryOp::Or, left, right)
}

pub fn _cond(cond: impl Into<Expr>, then: impl Into<Expr>, else_: impl Into<Expr>) -> Expr {
    Expr::CondExpr(Box::new(crate::CondExpr(cond.into(), then.into(), else_.into())))
}

fn _unary(op: UnaryOp, expr: impl Into<Expr>) -> Expr {
    let expr = expr.into();

    match expr {
        Expr::UnaryExpr(unary) => {
            let mut ops = Vec::from(unary.op);
            ops.push(op);
            Expr::UnaryExpr(Box::new(UnaryExpr{
                op: ops.into_boxed_slice(),
                expr: unary.expr,
            }))
        },
        _ => {
            Expr::UnaryExpr(Box::new(UnaryExpr{
                op: Box::new([op]),
                expr: expr,
            }))
        },
    }
}

pub fn _typeof(expr: impl Into<Expr>) -> Expr {
    _unary(UnaryOp::TypeOf, expr)
}

pub fn _pos(expr: impl Into<Expr>) -> Expr {
    _unary(UnaryOp::Pos, expr)
}

pub fn _neg(expr: impl Into<Expr>) -> Expr {
    _unary(UnaryOp::Neg, expr)
}

pub fn _not(expr: impl Into<Expr>) -> Expr {
    _unary(UnaryOp::Not, expr)
}

pub fn _bitnot(expr: impl Into<Expr>) -> Expr {
    _unary(UnaryOp::BitNot, expr)
}

fn _callpost(op: CallPostOp, expr: impl Into<Expr>) -> Expr {
    let expr = expr.into();

    match expr {
        Expr::CallExpr(call) => {
            let mut ops = Vec::from(call.post_ops);
            ops.push(op);
            Expr::CallExpr(Box::new(crate::CallExpr{
                expr: call.expr,
                post_ops: ops.into_boxed_slice(),
            }))
        },
        _ => {
            Expr::CallExpr(Box::new(crate::CallExpr{
                expr: expr,
                post_ops: Box::new([op]),
            }))
        },
    }
}

pub fn _index(expr: impl Into<Expr>, index: impl Into<Expr>) -> Expr {
    _callpost(CallPostOp::Index(index.into()), expr)
}

pub fn _member(expr: impl Into<Expr>, member: &str) -> Expr {
    _callpost(CallPostOp::Member(Rc::from(member)), expr)
}

pub fn _call(expr: impl Into<Expr>, args: &[Expr]) -> Expr {
    _callpost(CallPostOp::Call(Box::from(args)), expr)
}

pub fn _assign(left: impl Into<LValue>, right: impl Into<Expr>) -> Expr {
    Expr::Assignment(Box::new(Assignment(crate::AssignOp::Assign, left.into(), right.into())))
}

pub fn _add_assign(left: impl Into<LValue>, right: impl Into<Expr>) -> Expr {
    Expr::Assignment(Box::new(Assignment(crate::AssignOp::AssignAdd, left.into(), right.into())))
}

pub fn _var(name: &str) -> Expr {
    Expr::Variable(Box::new(Variable{
        name: Rc::from(name),
        pointer: Rc::new(OnceCell::new())
    }))
}

pub fn _function_raw(name: &str,scope: Option<crate::FunctionScope>, params: &[Pattern], body: impl Into<Block>) -> Function {
    crate::Function{
        name: crate::FunctionName::Named(Rc::from(name)),
        parameters: Box::from(params),
        body: crate::ExprOrBlock::Block(body.into()),
        scope: scope.map(Into::into),
    }
}

pub fn _function(name: &str,scope: Option<crate::FunctionScope>, params: &[Pattern], body: impl Into<Block>) -> Expr {
    Expr::Function(Box::new(crate::Function{
        name: crate::FunctionName::Named(Rc::from(name)),
        parameters: Box::from(params),
        body: crate::ExprOrBlock::Block(body.into()),
        scope: scope.map(Into::into),
    }))
}

pub fn _function_anonymous(params: &[Pattern], body: impl Into<Block>) -> Expr {
    Expr::Function(Box::new(crate::Function{
        name: crate::FunctionName::Anonymous,
        parameters: Box::from(params),
        body: crate::ExprOrBlock::Block(body.into()),
        scope: None,
    }))
}

pub fn arrow_expr(params: &[Pattern], body: impl Into<Expr>) -> Expr {
    Expr::Function(Box::new(crate::Function{
        name: crate::FunctionName::Arrow,
        parameters: Box::from(params),
        body: crate::ExprOrBlock::Expr(body.into()),
        scope: None,
    }))
}

pub fn arrow_block(params: &[Pattern], body: impl Into<Block>) -> Expr {
    Expr::Function(Box::new(crate::Function{
        name: crate::FunctionName::Arrow,
        parameters: Box::from(params),
        body: crate::ExprOrBlock::Block(body.into()),
        scope: None,
    }))
}

