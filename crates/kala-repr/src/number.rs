use std::mem::{MaybeUninit, transmute};

use crate::{integer::Integer, reference::Reference};

pub const POSITIVE_INFINITY: Number = Number {
    x0: 0xffff_ffff,
    x1: 0xffff_ffff,
    x2: 0xffff_ffff,
    x3: 0x7fff_ffff,
};

pub const NEGATIVE_INFINITY: Number = Number {
    x0: 0,
    x1: 0,
    x2: 0,
    x3: 0x8000_0000u32 as i32,
};

pub const NUMBER_ZERO: Number = Number {
    x0: 0,
    x1: 0,
    x2: 0,
    x3: 0,
};

#[cfg(target_endian="little")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub x0: u32,
    pub x1: u32,
    pub x2: u32,
    pub x3: i32,
}

impl Number {
    #[cfg(target_endian="little")] 
    fn assert_memory_sanity(&self) {
        unsafe {
            assert_eq!(transmute::<Number, i128>(Number{x0: 0, x1: 0, x2: 1, x3: 0}), 1i128);
            assert_eq!(transmute::<Number, i128>(Number{x0: u32::MAX-1, x1: u32::MAX, x2: u32::MAX, x3: i32::MIN}), i128::MIN+1);
        }
    }

    #[cfg(target_endian="little")]    
    pub fn new(i: i64, f: u64) -> Self {
        let mut result = MaybeUninit::<Number>::uninit();
        unsafe {
            result.as_mut_ptr().write(Number {
                x0: f as u32,
                x1: (f >> 32) as u32,
                x2: i as u32,
                x3: (i >> 32) as i32,
            });
            result.assume_init()
        } 
    }
}



impl Number {
    pub fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let (res0, overflow0) = self.x0.overflowing_add(other.x0);
        let (res1, overflow1) = self.x1.carrying_add(other.x1, overflow0);
        let (res2, overflow2) = self.x2.carrying_add(other.x2, overflow1);
        let (res3, overflow3) = self.x3.carrying_add(other.x3, overflow2);

        let res = Number {
            x0: res0,
            x1: res1,
            x2: res2,
            x3: res3,
        };

        (res, overflow3)
    }

    pub fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        let (res0, borrow0) = self.x0.overflowing_sub(other.x0);
        let (res1, borrow1) = self.x1.borrowing_sub(other.x1, borrow0);
        let (res2, borrow2) = self.x2.borrowing_sub(other.x2, borrow1);
        let (res3, borrow3) = self.x3.borrowing_sub(other.x3, borrow2);

        let res = Number {
            x0: res0,
            x1: res1,
            x2: res2,
            x3: res3,
        };

        (res, borrow3)
    }

    pub(crate) fn op_strict_equal_internal(&self, other: &Self) -> bool {
        self.x0 == other.x0
            && self.x1 == other.x1
            && self.x2 == other.x2
            && self.x3 == other.x3
    }
/* 
    #[cfg(target_pointer_width="32")]
    #[cfg(target_endian="little")]
    pub(crate) fn op_strict_equal_internal_integer(&self, other: &Integer) -> bool {
        if self.x0 != 0 || self.x1 != 0 {
            return false;
        }

        let integer_part: i64 = unsafe{transmute((self.x2, self.x3))};
        integer_part == other.unwrap().try_into().unwrap()
    }
*/
    pub(crate) fn op_strict_equal_internal_integer(&self, other: &Integer) -> bool {
        if self.x0 != 0 || self.x1 != 0 {
            return false;
        }

        let integer_part: i64 = unsafe{transmute((self.x2, self.x3))};
        integer_part == other.unwrap().try_into().unwrap()
    }

    pub(crate) fn op_less_than_internal(&self, other: &Self) -> Option<bool> {
        unimplemented!("op_less_than_internal")
    }

    pub(crate) fn op_less_than_internal_integer(&self, other: &Integer) -> Option<bool> {
        unimplemented!("op_less_than_internal_integer")
    }

    pub(crate) fn op_add_internal(&self, other: &Self) -> Option<Self> {
        unimplemented!("op_add_internal")
    }

    pub(crate) fn op_sub_internal(&self, other: &Self) -> Option<Self> {
        unimplemented!("op_sub_internal")
    }

    pub(crate) fn op_add_internal_integer(&self, other: &Integer) -> Option<Self> {
        unimplemented!("op_add_internal_integer")
    }

    pub(crate) fn op_neg(&self) -> Self {
        unimplemented!("op_neg")
    }

}

impl ToString for Number {
    fn to_string(&self) -> String {
        unimplemented!("Number::to_string")
    }
}

impl Into<Reference> for Number {
    fn into(self) -> Reference {
        unimplemented!("Number::into<Reference>")
    }
}