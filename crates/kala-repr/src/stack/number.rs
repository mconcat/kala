use std::{ops::{Deref, Add, Sub, Mul, Div}, net::AddrParseError};

use crate::SlotPointer;

use super::{SlotTag, pointer::np32, Slot};

pub const POSITIVE_INFINITY: i128 = 0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;

pub const NEGATIVE_INFINITY: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;

pub const NAN_SLOT: u64 = SlotTag::Number.attach(0xFFFF_FFFF_FFFF_FFFF);

#[repr(C)]
pub struct NumberSlot {
	value: i32,
	pointer: np32<i128>,
}

impl NumberSlot {
	pub fn new(value: i128) -> Self {
		Self{ value: 0, pointer: np32::new(value) }
	}

	pub fn new_inline(value: i32) -> Self {
		Self{ value, pointer: np32::null() }
	}

	pub const fn new_positive_infinity() -> Self {
		Self{ value: 0, pointer: np32::new(i128::MAX) }
	}

	pub const fn new_negative_infinity() -> Self {
		Self{ value: 0, pointer: np32::new(i128::MIN) }
	}
}

impl Into<Slot> for NumberSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Number) }
	}	
}

impl Deref for NumberSlot {
	type Target = i128;

	fn deref(&self) -> &Self::Target {
		if self.pointer.is_null() {
			&i128::from(self.value)
		} else {
			unsafe {&*self.pointer}
		}
	}
}

impl Add for NumberSlot {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_add(rhs.value);
			if overflow {
				return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let (res, overflow) = self_value.overflowing_add(rhs_value);
		if overflow {
			if res < 0 {
				return NumberSlot::new_positive_infinity()
			} else {
				return NumberSlot::new_negative_infinity()
			}
		}

		NumberSlot::new(res)
	}
}

impl Add for &NumberSlot {
	type Output = NumberSlot;

	fn add(self, rhs: Self) -> Self::Output {
		*self + *rhs
	}
}

impl Sub for NumberSlot {
	type Output = Self; 

	fn sub(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_sub(rhs.value);
			if overflow {
				//return NumberSlot::new(res as i128)
				unimplemented!("overflow")
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let (res, overflow) = self_value.overflowing_sub(rhs_value);
		if overflow {
			if res < 0 {
				return NumberSlot::new_positive_infinity()
			} else {
				return NumberSlot::new_negative_infinity()
			}
		}

		NumberSlot::new(res)
	}
}

impl Mul for NumberSlot {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_mul(rhs.value);
			if overflow {
				unimplemented!("overflow")
				//return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let self_lo = self_value & 0xFFFF_FFFF_FFFF_FFFF;
		let self_hi = self_value >> 64;
		let rhs_lo = rhs_value & 0xFFFF_FFFF_FFFF_FFFF;
		let rhs_hi = rhs_value >> 64;

		let mut hihi = self_hi * rhs_hi;
		if hihi>>64 != 0 {
			unimplemented!("overflow")
		}

		let lolo= self_lo * rhs_lo;
		let lohi = self_hi * rhs_lo;
		let hilo = self_lo * rhs_hi;

		let mut hihi_carry = false;
		let (mut mid, mut mid_carry) = lohi.overflowing_add(hilo);
		if mid_carry {
			(hihi, hihi_carry) = hihi.overflowing_add(1);
			if hihi>>64 != 0 || hihi_carry {
				unimplemented!("overflow")
			}
		}
		(mid, mid_carry) = mid.overflowing_add(lolo>>64);
		if mid_carry {
			(hihi, hihi_carry) = hihi.overflowing_add(1);
			if hihi>>64 != 0 || hihi_carry {
				unimplemented!("overflow")
			}
		}

		NumberSlot::new(mid)
	}
}

impl Div for NumberSlot {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			if rhs.value == 0 {
				unimplemented!("divide by zero, TODO: throw")
			}

			if self.value % rhs.value == 0 {
				let (res, overflow) = self.value.overflowing_div(rhs.value);
				if overflow {
					unimplemented!("overflow")
				}
				return NumberSlot::new_inline(res)
			}

			// else, fallthrough
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		unimplemented!("long division")
	}
}
