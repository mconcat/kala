use std::{ops::{Add, Sub, Mul, Div, Neg}, mem::transmute};

use crate::{inline_numeric::inline::InlineNumericSlot, memory::alloc::Ref, slot::{SlotTag, Slot}};

pub struct BigintSlot(pub(crate) InlineNumericSlot<{SlotTag::Bigint}>);

impl BigintSlot {
    pub fn new(value: isize) -> Self {
        Self(InlineNumericSlot::new(value))
    }

    pub fn promote(self) -> BigintSlot {
        unimplemented!("promote")
    }
}

impl Add for BigintSlot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BigintSlot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for BigintSlot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for BigintSlot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Neg for BigintSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl PartialEq for BigintSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for BigintSlot {
    fn to_string(&self) -> String {
        self.0.unwrap().to_string()
    }
}

impl Into<Slot> for BigintSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}