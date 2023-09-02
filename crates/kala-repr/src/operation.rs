use std::{ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr}, mem::transmute};

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
    pub fn strict_equal_internal(&self, other: &Self) -> bool {
        if self.value == other.value && self.pointer.0 == other.pointer.0 {
            return true
        }

        match (self.into_typed(), other.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs == rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs == rhs,
            (TypedSlot::String(lhs), TypedSlot::String(rhs)) => lhs == rhs,
            (TypedSlot::Reference(lhs), TypedSlot::Reference(rhs)) => lhs.pointer.0 == rhs.pointer.0,
            (_, _) => false,
        }
    } 

    pub fn strict_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            TRUE
        } else {
            FALSE
        }
    }

    pub fn strict_not_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            FALSE
        } else {
            TRUE
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
            TypedSlot::Number(slot) => {
                if slot.is_inline() {
                    println!("inline number");
                    slot.value.to_string()
                } else {
                    println!("non-inline number");
                    slot.pointer.to_string() // TODO
                }
            },
            TypedSlot::Bigint(slot) => {
                unimplemented!("bigint to string")
            },
            TypedSlot::String(slot) => {
                slot.to_string()
            },
            TypedSlot::Reference(_) => {
                if self.is_undefined() {
                    "undefined".to_string()
                } else if self.is_null() {
                    "null".to_string()
                } else if self.is_true() {
                    "true".to_string()
                } else if self.is_false() {
                    "false".to_string()
                } else {
                    unimplemented!("reference to string")
                }
            },
            TypedSlot::Uninitialized => {
                unimplemented!("uninitialized to string")
            },
        }
    }
}