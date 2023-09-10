use std::{ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr}, mem::transmute};

use crate::{slot::{Slot, TypedSlot}, number::NumberSlot, bigint::BigintSlot};

impl Add for Slot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs + rhs).into(),
            (TypedSlot::Number(lhs), TypedSlot::InlineNumber(rhs)) => (lhs + rhs.promote()).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::Number(rhs)) => ((lhs.promote()) + rhs).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::InlineNumber(rhs)) => (lhs + rhs).into(),

            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs + rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs + (rhs.promote())).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::Bigint(rhs)) => ((lhs.promote()) + rhs).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs + rhs).into(),

            _ => unimplemented!("throw error"),
        }
    }
}

impl Sub for Slot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs - rhs).into(),
            (TypedSlot::Number(lhs), TypedSlot::InlineNumber(rhs)) => (lhs - rhs.promote()).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::Number(rhs)) => ((lhs.promote()) - rhs).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::InlineNumber(rhs)) => (lhs - rhs).into(),

            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs - rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs - (rhs.promote())).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::Bigint(rhs)) => ((lhs.promote()) - rhs).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs - rhs).into(),

            _ => unimplemented!("throw error"),
        }
    }
}

impl Mul for Slot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs * rhs).into(),
            (TypedSlot::Number(lhs), TypedSlot::InlineNumber(rhs)) => (lhs * rhs.promote()).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::Number(rhs)) => ((lhs.promote()) * rhs).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::InlineNumber(rhs)) => (lhs * rhs).into(),

            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs * rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs * (rhs.promote())).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::Bigint(rhs)) => ((lhs.promote()) * rhs).into(),
            (TypedSlot::InlineBigint(lhs), TypedSlot::InlineBigint(rhs)) => (lhs * rhs).into(),

            _ => unimplemented!("throw error"),
        }
    }
}

impl Div for Slot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs / rhs).into(), 
            (TypedSlot::Number(lhs), TypedSlot::InlineNumber(rhs)) => (lhs / rhs.promote()).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::Number(rhs)) => ((lhs.promote()) / rhs).into(),
            (TypedSlot::InlineNumber(lhs), TypedSlot::InlineNumber(rhs)) => (lhs / rhs).into(),

            _ => unimplemented!("throw error"),
        }
    }
}

impl Neg for Slot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.into_typed() {
            TypedSlot::Number(num) => (-num).into(),
            TypedSlot::InlineNumber(num) => (-num).into(),
            TypedSlot::Bigint(num) => (-num).into(),
            TypedSlot::InlineBigint(num) => (-num).into(),

            _ => unimplemented!("throw error"),
        }
    }
}

impl BitAnd for Slot {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            

            _ => unimplemented!("throw error"),
        }
    }
}

impl BitOr for Slot {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitXor for Slot {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
           
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shl for Slot {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            

            _ => unimplemented!("throw error"),
        }
    }
}

impl Shr for Slot {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
           
            _ => unimplemented!("throw error"),
        }
    }
}

impl Slot {
    pub fn strict_equal_internal(&self, other: &Self) -> bool {
        match (self.into_typed(), other.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::InlineNumber(lhs), TypedSlot::InlineNumber(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::InlineBigint(lhs), TypedSlot::InlineBigint(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::String(lhs), TypedSlot::String(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::Reference(lhs), TypedSlot::Reference(rhs)) => {
                lhs == rhs
            },

            (TypedSlot::InlineReference(lhs), TypedSlot::InlineReference(rhs)) => {
                lhs == rhs
            },

            (_, _) => false,
        }
    } 

    pub fn strict_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            Slot::new_true()
        } else {
            Slot::new_false()
        }
    }

    pub fn strict_not_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            Slot::new_false()
        } else {
            Slot::new_true()
        }
    }

    pub fn less_than(self, other: Self) -> Self {
        match (self.into_typed(), other.into_typed()) {
            
            
            (_, _) => panic!("TODO: type error")
        }
    }

    pub fn less_than_or_equal(self, other: Self) -> Self {
        unimplemented!("lte")
    }

    pub fn greater_than_or_equal(self, other: Self) -> Self {
        unimplemented!("gte")
    }

    pub fn greater_than(self, other: Self) -> Self {
        unimplemented!("gt")
    }    
}

impl ToString for Slot {
    fn to_string(&self) -> String {
        match self.into_typed() {
            TypedSlot::Number(num) => {
                num.to_string()
            },
            TypedSlot::InlineNumber(num) => {
                num.to_string()
            },
            TypedSlot::Bigint(num) => {
                unimplemented!("bigint to string")
            },
            TypedSlot::InlineBigint(num) => {
                num.to_string()
            },
            TypedSlot::String(string) => {
                string.to_string()
            },
            TypedSlot::Reference(reference) => {
                unimplemented!("reference to string")
            },
            TypedSlot::InlineReference(reference) => {
                reference.to_string()
            },
        }
    }
}