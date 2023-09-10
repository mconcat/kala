use std::{ops::{Add, Sub, Mul, Div, Neg}, mem::transmute};

use crate::{slot::{SlotTag, Slot}, inline_numeric::inline::InlineNumericSlot};

use super::NumberSlot;

pub struct InlineNumberSlot(pub(crate) InlineNumericSlot<{SlotTag::InlineNumber}>);

impl InlineNumberSlot {
    pub fn promote(self) -> NumberSlot {
        self.into() 
    }
}

impl Into<NumberSlot> for InlineNumberSlot {
    fn into(self) -> NumberSlot {
        unsafe{transmute(self)}
    }
}

impl Into<Slot> for InlineNumberSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}

impl Add for InlineNumberSlot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        InlineNumberSlot(self.0 + rhs.0)
    }
}

impl Sub for InlineNumberSlot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        InlineNumberSlot(self.0 - rhs.0)
    }
}

impl Mul for InlineNumberSlot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        InlineNumberSlot(self.0 * rhs.0)
    }
}

impl Div for InlineNumberSlot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        InlineNumberSlot(self.0 / rhs.0)
    }
}

impl Neg for InlineNumberSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        InlineNumberSlot(-self.0)
    }
}

impl PartialEq for InlineNumberSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for InlineNumberSlot {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}