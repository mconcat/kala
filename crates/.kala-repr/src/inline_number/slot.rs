use std::{ops::{Add, Sub, Mul, Div, Neg, Deref}, mem::transmute};

use crate::{slot::{SlotTag, Slot}, inline_numeric::inline::InlineNumericSlot};

pub struct NumberSlot(pub(crate) InlineNumericSlot<{SlotTag::Number}>);

impl NumberSlot {
    pub fn promote(self) -> NumberSlot {
        self.into() 
    }

    pub fn new(value: isize) -> Self {
        Self(InlineNumericSlot::new(value))
    }
}

impl Into<Slot> for NumberSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}

impl Add for NumberSlot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        NumberSlot(self.0 + rhs.0)
    }
}

impl Sub for NumberSlot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NumberSlot(self.0 - rhs.0)
    }
}

impl Mul for NumberSlot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        NumberSlot(self.0 * rhs.0)
    }
}

impl Div for NumberSlot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        NumberSlot(self.0 / rhs.0)
    }
}

impl Neg for NumberSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        NumberSlot(-self.0)
    }
}

impl PartialEq for NumberSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for NumberSlot {
    fn to_string(&self) -> String {
        self.0.unwrap().to_string()
    }
}