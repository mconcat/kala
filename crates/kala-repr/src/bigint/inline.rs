use std::{ops::{Add, Sub, Mul, Div, Neg}, mem::transmute};

use crate::{inline_numeric::inline::InlineNumericSlot, memory::alloc::Ref, slot::{SlotTag, Slot}};

use super::BigintSlot;

pub struct InlineBigintSlot(pub(crate) InlineNumericSlot<{SlotTag::InlineBigint}>);

impl InlineBigintSlot {
    pub fn new(value: isize) -> Self {
        Self(InlineNumericSlot::new(value))
    }

    pub fn promote(self) -> BigintSlot {
        unimplemented!("promote")
    }
}

impl Add for InlineBigintSlot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for InlineBigintSlot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for InlineBigintSlot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for InlineBigintSlot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Neg for InlineBigintSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl PartialEq for InlineBigintSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for InlineBigintSlot {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<Slot> for InlineBigintSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}