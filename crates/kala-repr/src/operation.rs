use std::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr};

use crate::slot::{Slot, TypedSlot, TRUE, FALSE};

impl Add for Slot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs + rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs + rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Sub for Slot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs - rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs - rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Mul for Slot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs * rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs * rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Div for Slot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs / rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs / rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Neg for Slot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.into_typed() {
            TypedSlot::Number(lhs) => (-lhs).into(),
            TypedSlot::Bigint(lhs) => (-lhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitAnd for Slot {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs & rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs & rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitOr for Slot {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs | rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs | rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitXor for Slot {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs ^ rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs ^ rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shl for Slot {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs << rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs << rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shr for Slot {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => (lhs >> rhs).into(),
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => (lhs >> rhs).into(),
            _ => unimplemented!("throw error"),
        }
    }
}

impl Slot {
    pub fn strict_equal(self, other: Self) -> Self {
        if self.0 == other.0 {
            TRUE
        } else {
            FALSE
        }
    }

    pub fn strict_not_equal(self, other: Self) -> Self {
        if self.0 != other.0 {
            TRUE
        } else {
            FALSE
        }
    }

    pub fn less_than(self, other: Self) -> Self {
        match (self.into_typed(), other.into_typed()) {
            (TypedSlot::Number(x), TypedSlot::Number(y)) => x.less_than(y).into(),
            (TypedSlot::Bigint(x), TypedSlot::Bigint(y)) => x.less_than(y).into(),
            (TypedSlot::String(x), TypedSlot::String(y)) => unimplemented!("string comp lt"),
            (_, _) => panic!("TODO: type error")
        }
    }

    pub fn less_than_equal(self, other: Self) -> Self {
        unimplemented!("lte")
    }

    pub fn greater_than_equal(self, other: Self) -> Self {
        unimplemented!("gte")
    }

    pub fn greater_than(self, other: Self) -> Self {
        unimplemented!("gt")
    }    
}