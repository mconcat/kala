use std::mem::ManuallyDrop;

use crate::slot::{SlotInteger, Slot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub isize); // tagged with 0b0001

const INTEGER_MASK: isize = !0b1111;

impl Integer {
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
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<Slot> for Integer {
    fn into(self) -> Slot {
        Slot{ integer: ManuallyDrop::new(SlotInteger(self)) }
    }
}