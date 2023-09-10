use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::slot::SlotTag;

use super::inline::InlineNumericSlot;

impl<const Tag: SlotTag> Add for InlineNumericSlot<Tag> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (res, overflow) = (*self).overflowing_add(*rhs);
        if overflow {
            unimplemented!("overflow")
        }

        InlineNumericSlot::new(res)
    }
}

impl<const Tag: SlotTag> Sub for InlineNumericSlot<Tag> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (res, overflow) = (*self).overflowing_sub(*rhs);
        if overflow {
            unimplemented!("overflow")
        }

        InlineNumericSlot::new(res)
    }
}

impl<const Tag: SlotTag> Mul for InlineNumericSlot<Tag> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (res, overflow) = (*self).overflowing_mul(*rhs);
        if overflow {
            unimplemented!("overflow")
        }

        InlineNumericSlot::new(res)
    }
}

impl<const Tag: SlotTag> Div for InlineNumericSlot<Tag> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (res, overflow) = (*self).overflowing_div(*rhs);
        if overflow {
            unimplemented!("overflow")
        }

        InlineNumericSlot::new(res)
    }
}

impl<const Tag: SlotTag> Neg for InlineNumericSlot<Tag> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (res, overflow) = (*self).overflowing_neg();
        if overflow {
            unimplemented!("overflow")
        }

        InlineNumericSlot::new(res)
    }
}

impl<const Tag: SlotTag> PartialEq for InlineNumericSlot<Tag> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}