// "Justin"... sort of 

use std::fmt::{Binary, Debug};


// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence
#[derive(PartialEq, Clone)]
pub enum BinaryOp {
    // 3
    Or, // Left
    Coalesce, // Left

    // 4
    And, // Left

    // 5
    BitOr, // Left

    // 6
    BitXor, // Left

    // 7
    BitAnd, // Left

    // 8 
    StrictEqual, // Left
    StrictNotEqual, // Left

    // 9 
    LessThan, // Left
    LessThanEqual, // Left
    GreaterThan, // Left
    GreaterThanEqual, // Left

    // 10
    BitLeftShift, // Left
    BitRightShift, // Left
    BitUnsignedRightShift, // Left

    // 11
    Add, // Left
    Sub, // Left

    // 12
    Mul, // Left
    Div, // Left
    Mod, // Left

    // 13
    Pow, // Right
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Or => write!(f, "||"),
            BinaryOp::Coalesce => write!(f, "??"),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::BitOr => write!(f, "|"),
            BinaryOp::BitXor => write!(f, "^"),
            BinaryOp::BitAnd => write!(f, "&"),
            BinaryOp::StrictEqual => write!(f, "==="),
            BinaryOp::StrictNotEqual => write!(f, "!=="),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::LessThanEqual => write!(f, "<="),
            BinaryOp::GreaterThan => write!(f, ">"),
            BinaryOp::GreaterThanEqual => write!(f, ">="),
            BinaryOp::BitLeftShift => write!(f, "<<"),
            BinaryOp::BitRightShift => write!(f, ">>"),
            BinaryOp::BitUnsignedRightShift => write!(f, ">>>"),
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Pow => write!(f, "**"),
        }
    
    }
}

#[derive(PartialEq, Clone)]
pub enum UnaryOp { // preOp
    TypeOf,
    Pos,
    Neg,
    BitNot,
    Not,
}

impl Debug for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::TypeOf => write!(f, "typeof "),
            UnaryOp::Pos => write!(f, "+"),
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::BitNot => write!(f, "~"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}