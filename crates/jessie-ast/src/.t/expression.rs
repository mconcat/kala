use crate::{BinaryOp, CallExpr, CallPostOp};
use std::{ops::{Add, Sub, Mul, Div, Rem, Index, IndexMut, Neg, Not}, rc::Rc};

// Literals

pub fn lit(lit: crate::DataLiteral) -> crate::Expr {
    crate::Expr::DataLiteral(Box::new(lit))
}

pub fn bool(b: bool) -> crate::Expr {
    if b {
        crate::Expr::DataLiteral(Box::new(crate::DataLiteral::True))
    } else {
        crate::Expr::DataLiteral(Box::new(crate::DataLiteral::False))
    }
}

pub fn null() -> crate::Expr {
    crate::Expr::DataLiteral(Box::new(crate::DataLiteral::Null))
}

pub fn undefined() -> crate::Expr {
    crate::Expr::DataLiteral(Box::new(crate::DataLiteral::Undefined))
}

pub fn number(n: i64) -> crate::Expr {
    crate::Expr::DataLiteral(Box::new(crate::DataLiteral::Integer(n)))
}

pub fn string(s: &str) -> crate::Expr {
    crate::Expr::DataLiteral(Box::new(crate::DataLiteral::String(Rc::from(s))))
}

// Arrays
#[macro_export]
macro_rules! array {
    (building, [$($elem:expr),*]) => {
        crate::Expr::Array(Box::new(crate::Array(vec![$($elem)*])))
    };

    // element
    (building, [$($elem:expr),*], $e:expr, $($rest:tt)+) => {
        array!(building, [$($elem),*, $e], $($rest)+)
    };

    ($e:expr, $($rest:tt)+) => {
        array!(building, [], $e, $($rest)+)
    };

    // spread
    (building, [$($elem:expr),*], ...$e:expr, $($rest:tt)+) => {
        array!(building, [$($elem),*, crate::Expr::Spread($e.into())], $($rest)+)
    };

    (...$e:expr, $($rest:tt)+) => {
        array!(building, [], ...$e, $($rest)+)
    };
}

// Records
#[macro_export]
macro_rules! record {
    (building, [$($prop:expr),*]) => {
        crate::Expr::Record(Box::new(crate::Record(vec![$($prop)*])))
    };
    // keyvalue
    (building, [$($prop:expr),*], $k:ident => $v:expr, $($rest:tt)+) => {
        record!(building, [$($prop),*, crate::PropDef::KeyValue(
            Box::new(crate::Field::new_dynamic(stringify!($k))),
            $v,
        )], $($rest)+)
    };

    ($k:expr => $v:expr, $($rest:tt)+) => {
        record!(building, [], $k => $v, $($rest)+)
    }; 

    // shorthand
    (building, [$($prop:expr),*], $k:expr, $($rest:tt)+) => {
        record!(building, [$($prop),*, crate::PropDef::Shorthand(
            Box::new(crate::Field::new_dynamic(stringify!($k))),
            Box::new(crate::VariableCell::uninitialized(SharedString::from_string(stringify!($k)))),
        )], $($rest)+)
    };

    ($k:expr, $($rest:tt)+) => {
        record!(building, [], $k, $($rest)+)
    };

    // spread
    (building, [$($prop:expr),*], ...$e:expr, $($rest:tt)+) => {
        record!(building, [$($prop),*, crate::PropDef::Spread($e.into())], $($rest)+)
    };

    (...$e:expr, $($rest:tt)+) => {
        record!(building, [], ...$e, $($rest)+)
    };
}

// Assignment
impl crate::LValue {
    pub fn assign(self, rhs: crate::Expr) -> crate::Expr {
        crate::Expr::Assignment(Box::new(crate::Assignment(crate::AssignOp::Assign, self, rhs)))
    }
}

// BinaryExpr
pub fn bin(op: BinaryOp, l: crate::Expr, r: crate::Expr) -> crate::Expr {
    crate::Expr::BinaryExpr(Box::new(crate::BinaryExpr(op, l, r)))
}


impl Add for crate::Expr {
    type Output = crate::Expr;

    fn add(self, other: Self) -> Self::Output {
        bin(BinaryOp::Add, self, other)
    }
}

impl Sub for crate::Expr {
    type Output = crate::Expr;

    fn sub(self, other: Self) -> Self::Output {
        bin(BinaryOp::Sub, self, other)
    }
}

impl Mul for crate::Expr {
    type Output = crate::Expr;

    fn mul(self, other: Self) -> Self::Output {
        bin(BinaryOp::Mul, self, other)
    }
}

impl Div for crate::Expr {
    type Output = crate::Expr;

    fn div(self, other: Self) -> Self::Output {
        bin(BinaryOp::Div, self, other)
    }
}

impl Rem for crate::Expr {
    type Output = crate::Expr;

    fn rem(self, other: Self) -> Self::Output {
        bin(BinaryOp::Mod, self, other)
    }
}

impl crate::Expr {
    pub fn and(self, other: crate::Expr) -> crate::Expr {
        bin(BinaryOp::And, self, other)
    }

    pub fn or(self, other: crate::Expr) -> crate::Expr {
        bin(BinaryOp::Or, self, other)
    }
}

// CondExpr

impl crate::Expr {
    pub fn cond(self, t: crate::Expr, f: crate::Expr) -> crate::Expr {
        crate::Expr::CondExpr(Box::new(crate::CondExpr(self, t, f)))
    }
}

// UnaryExpr

impl Not for crate::Expr {
    type Output = crate::Expr;

    fn not(mut self) -> Self::Output {
        match self {
            crate::Expr::UnaryExpr(ue) => {
                let mut res = ue.clone();
                res.op.push(crate::UnaryOp::Not);
                crate::Expr::UnaryExpr(res)
            }
            _ => crate::Expr::UnaryExpr(Box::new(crate::UnaryExpr {
                op: vec![crate::UnaryOp::Not],
                expr: self,
            }))
        }
    }
}

impl Neg for crate::Expr {
    type Output = crate::Expr;

    fn neg(self) -> Self::Output {
        match self {
            crate::Expr::UnaryExpr(ue) => {
                let mut res = ue.clone();
                res.op.push(crate::UnaryOp::Neg);
                crate::Expr::UnaryExpr(res)
            }
            _ => crate::Expr::UnaryExpr(Box::new(crate::UnaryExpr {
                op: vec![crate::UnaryOp::Neg],
                expr: self,
            }))
        }
    }
}

// CallExpr

impl crate::Expr {
    pub fn index(self, index: impl Into<Self>) -> Self {
        match self {
            crate::Expr::CallExpr(ce) => {
                let mut result = ce.clone();
                result.post_ops.push(crate::CallPostOp::Index(index.into()));
                crate::Expr::CallExpr(result)
            },
            _ => crate::Expr::CallExpr(Box::new(crate::CallExpr {
                expr: self,
                post_ops: vec![crate::CallPostOp::Index(index.into())],
            }))
        }
    }

    pub fn prop(self, prop: &str) -> Self {
        match self {
            crate::Expr::CallExpr(ce) => {
                let mut result = ce.clone();
                result.post_ops.push(crate::CallPostOp::Member(Rc::from(prop)));
                crate::Expr::CallExpr(result)
            },
            _ => crate::Expr::CallExpr(Box::new(crate::CallExpr {
                expr: self,
                post_ops: vec![crate::CallPostOp::Member(Rc::from(prop))],
            }))
        }
    }

    pub fn call(self, args: &[Self]) -> Self {
        match self {
            crate::Expr::CallExpr(ce) => {
                let mut result = ce.clone();
                result.post_ops.push(crate::CallPostOp::Call(args.to_vec()));
                crate::Expr::CallExpr(result)
            },
            _ => crate::Expr::CallExpr(Box::new(crate::CallExpr {
                expr: self,
                post_ops: vec![crate::CallPostOp::Call(args.to_vec())],
            }))
        }
    }
}

impl crate::LValue {
    pub fn index(self, index: impl Into<crate::Expr>) -> crate::LValue {
        match self {
            crate::LValue::CallLValue(mut clv) => {
                clv.post_ops.push(crate::LValueCallPostOp::Index(index.into()));
                crate::LValue::CallLValue(clv)
            },
            _ => crate::LValue::CallLValue(Box::new(crate::CallLValue {
                expr: self.into(),
                post_ops: vec![crate::LValueCallPostOp::Index(index.into())],
            }))
        }
    }

    pub fn prop(self, prop: &str) -> crate::LValue {
        match self {
            crate::LValue::CallLValue(mut clv) => {
                clv.post_ops.push(crate::LValueCallPostOp::Member(Rc::from(prop)));
                crate::LValue::CallLValue(clv)
            },
            _ => crate::LValue::CallLValue(Box::new(crate::CallLValue {
                expr: self.into(),
                post_ops: vec![crate::LValueCallPostOp::Member(Rc::from(prop))],
            }))
        }
    }
}

// parened

pub fn paren(e: crate::Expr) -> crate::Expr {
    crate::Expr::ParenedExpr(Box::new(e))
}

// Variable

pub struct Variable(pub Rc<str>);
/*
impl Into<crate::LValue> for Variable {
    fn into(self) -> crate::LValue {
        crate::LValue::Variable(Box::new(crate::VariableCell::uninitialized(self.0)))
    }
}


impl Into<crate::Pattern> for Variable {
    fn into(self) -> crate::Pattern {
        crate::Pattern::Variable(Box::new(crate::VariableCell::uninitialized(self.0)))
    }
}
*/

impl Into<crate::Expr> for Variable {
    fn into(self) -> crate::Expr {
        crate::Expr::Variable(Box::new(crate::VariableCell::uninitialized(self.0)))
    }
}


impl Add for Variable {
    type Output = crate::Expr;

    fn add(self, other: Self) -> Self::Output {
        bin(BinaryOp::Add, self.into(), other.into())
    }
}

#[macro_export]
macro_rules! var {
    ($name:ident) => {
        Variable(SharedString::from_str(stringify!($name)))
    };
}