use std::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr};

use crate::slot::{Slot, TypedSlot};

impl Add for Slot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs + rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs + rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Sub for Slot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs - rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs - rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Mul for Slot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs * rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs * rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Div for Slot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs / rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs / rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Neg for Slot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.into_typed() {
            TypedSlot::Number(lhs) => -lhs,
            TypedSlot::Bigint(lhs) => -lhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitAnd for Slot {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs & rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs & rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitOr for Slot {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs | rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs | rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitXor for Slot {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs ^ rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs ^ rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shl for Slot {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs << rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs << rhs,
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shr for Slot {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            (TypedSlot::Number(lhs), TypedSlot::Number(rhs)) => lhs >> rhs,
            (TypedSlot::Bigint(lhs), TypedSlot::Bigint(rhs)) => lhs >> rhs,
            _ => unimplemented!("throw error"),
        }
    }
}