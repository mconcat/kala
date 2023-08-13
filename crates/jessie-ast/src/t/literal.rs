use std::{ops::{Add, Sub, Mul, Div, Rem}, fmt::Binary};

use utils::SharedString;

use crate::BinaryOp;

fn binary_expr(op:BinaryOp, x: crate::DataLiteral, y: crate::DataLiteral) -> crate::Expr {
    crate::Expr::BinaryExpr(Box::new(crate::BinaryExpr(op, crate::Expr::DataLiteral(Box::new(x)), crate::Expr::DataLiteral(Box::new(y)))))
}

impl Add for crate::DataLiteral {
    type Output = crate::Expr;

    fn add(self, other: Self) -> Self::Output {
        binary_expr(BinaryOp::Add, self, other)
    }
}

impl Sub for crate::DataLiteral {
    type Output = crate::Expr;

    fn sub(self, other: Self) -> Self::Output {
        binary_expr(BinaryOp::Sub, self, other)
    }
}

impl Mul for crate::DataLiteral {
    type Output = crate::Expr;

    fn mul(self, other: Self) -> Self::Output {
        binary_expr(BinaryOp::Mul, self, other)
    }
}

impl Div for crate::DataLiteral {
    type Output = crate::Expr;

    fn div(self, other: Self) -> Self::Output {
        binary_expr(BinaryOp::Div, self, other)
    }
}

impl Rem for crate::DataLiteral {
    type Output = crate::Expr;

    fn rem(self, other: Self) -> Self::Output {
        binary_expr(BinaryOp::Mod, self, other)
    }
}

impl From<i64> for crate::Expr {
    fn from(n: i64) -> Self {
        crate::Expr::DataLiteral(Box::new(crate::DataLiteral::Integer(n)))
    }
}

impl From<&str> for crate::DataLiteral {
    fn from(s: &str) -> Self {
        crate::DataLiteral::String(s.to_string().into())
    }
}

impl From<bool> for crate::DataLiteral {
    fn from(b: bool) -> Self {
       if b { crate::DataLiteral::True } else { crate::DataLiteral::False }
    }
}

pub struct Variable {
    pub name: SharedString,
}