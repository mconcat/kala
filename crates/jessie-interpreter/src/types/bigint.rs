use std::ops::Add;

use crate::{memory::memory::Pointer, slot::Slot};

pub struct Bigint {
    sign: i32,
    abs: Pointer<[u64]>
}

impl From<Slot> for Bigint {
    fn from(slot: Slot) -> Self {
        let sign = slot.value.signum();
        let ptr = slot.get_pointer();
        if ptr.is_null() {
            Self {
                sign,
                abs: Pointer::null(),
            }
        } else {
            let ptr = unsafe { std::slice::from_raw_parts_mut(ptr, slot.value.abs() as usize) };
            Self {
                sign,
                abs: Pointer::new(ptr)
            }
        }
    }
}

impl Add for Bigint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Bigint::new();
        let mut carry = 0;
        let mut i = 0;
        while i < self.abs.len() || i < rhs.abs.len() {
            let a = self.abs.get(i).unwrap_or(0);
            let b = rhs.abs.get(i).unwrap_or(0);
            let (sum, carry1) = a.overflowing_add(b);
            let (sum, carry2) = sum.overflowing_add(carry);
            carry = carry1 as u64 + carry2 as u64;
            result.abs.push(sum);
            i += 1;
        }
        if carry != 0 {
            result.abs.push(carry);
        }
        result
    }
}