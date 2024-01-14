use std::{mem::ManuallyDrop, fmt::Debug};

use crate::{slot::{SlotInteger, Slot}, number::Number};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub isize); // tagged with 0b0001

const INTEGER_MASK: isize = !0b1111;

impl Integer {
    #[cfg(target_pointer_width="64")]
    pub fn new(x: i64) -> Option<Self> {
        if x>>60 == 0 || x>>60 == -1 {
            let mut res = Integer((x << 4) as isize);
            res.tag();
            Some(res)
        } else {
            None
        }
    }

    #[cfg(target_pointer_width="32")]
    pub fn new(x: i64) -> Option<Self> {
        if x>>28 == 0 || x>>28 == -1 {
            let mut res = Integer((x << 4) as isize);
            res.tag();
            res
        } else {
            None
        }
    }
    fn tag(&mut self) {
        self.0 &= INTEGER_MASK;
        self.0 |= 0b0001;
    }

    pub fn unwrap(&self) -> isize {
        (self.0 & INTEGER_MASK) >> 4
    }

    pub fn overflowing_add(self, other: Self) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_add(other.0);
        let mut res = Integer(x);
        res.tag();
        (res, overflow)
    }

    pub fn carrying_add(self, other: Self, mut overflow: bool) -> (Self, bool) {
        let (x, o) = self.0.overflowing_add(other.0);
        let mut res = Integer(x);
        res.tag();
        overflow |= o;
        (res, overflow)
    }

    pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_sub(other.0);
        let mut res = Integer(x);
        res.tag();
        (res, overflow)
    }

    pub fn borrowing_sub(self, other: Self, mut borrow: bool) -> (Self, bool) {
        let (x, o) = self.0.borrowing_sub(other.0, borrow);
        let mut res = Integer(x);
        res.tag();
        borrow |= o;
        (res, borrow)
    }

    pub(crate) fn op_strict_equal_internal(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    pub(crate) fn op_less_than_internal(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    pub fn op_add_internal(&self, other: &Self) -> Option<Self> {
        let (x, overflow) = self.0.overflowing_add(other.0);
        if overflow {
            None
        } else {
            let mut res = Integer(x);
            res.tag();
            Some(res)
        }
    }

    pub(crate) fn op_neg(&self) -> Self {
        let mut res = Integer(-self.0);
        res.tag();
        res
    }

    #[cfg(target_pointer_width="64")]
    pub(crate) fn overflowing_mul(self, other: Self) -> (isize, isize) {
        let x = self.unwrap() as i128;
        let y = other.unwrap() as i128;
        let res = x * y;
        (res as isize, (res >> 64) as isize)
    }

    #[cfg(target_pointer_width="32")]
    pub(crate) fn overflowing_mul(self, other: Self) -> (isize, isize) {
        let x = self.unwrap() as i64;
        let y = other.unwrap() as i64;
        let res = x * y;
        (res as isize, (res >> 32) as isize)
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.unwrap().to_string()
    }
}

impl Debug for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Integer")
            .field(&self.unwrap())
            .finish()
    }

}

impl Into<Slot> for Integer {
    fn into(self) -> Slot {
        Slot{ integer: ManuallyDrop::new(SlotInteger(self)) }
    }
}

impl Into<Number> for Integer {
    fn into(self) -> Number {
        Number::new(
            self.unwrap() as i64,
            0,
        )
    }
}