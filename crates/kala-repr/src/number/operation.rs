use std::{ops::{Deref, Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr}};

use super::NumberSlot;

impl Add for NumberSlot {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let (res, overflow) = (*self).overflowing_add(*rhs);
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
/* 
impl Add for &NumberSlot {
	type Output = NumberSlot;

	fn add(self, rhs: Self) -> Self::Output {
		*self + *rhs
	}
}
*/

impl Sub for NumberSlot {
	type Output = Self; 

	fn sub(self, rhs: Self) -> Self::Output {
		let (res, overflow) = (*self).overflowing_sub(*rhs);
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
		// shortcut for non-fractinal multiplication
		if *self << 64 == 0 && *rhs << 64 == 0 {
			let res = (*self >> 64 * *rhs >> 64);

			// check overflow
			if res >> 64 == 0 {
				return NumberSlot::new(res)
			}

			if res < 0 {
				return NumberSlot::new_positive_infinity()
			} else {
				return NumberSlot::new_negative_infinity()
			}
		}


		unimplemented!("long multiplication");
	}
}

impl Div for NumberSlot {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		unimplemented!("long division")
	}
}

impl Neg for NumberSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
		NumberSlot::new(-*self)
    }
}

impl BitAnd for NumberSlot {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitand")
	}
}

impl BitOr for NumberSlot {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitor")
	}
}

impl BitXor for NumberSlot {
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitxor")
	}
}

impl Shl for NumberSlot {
	type Output = Self;

	fn shl(self, rhs: Self) -> Self::Output {
		unimplemented!("number shl")
	}
}

impl Shr for NumberSlot {
	type Output = Self;

	fn shr(self, rhs: Self) -> Self::Output {
		unimplemented!("number shr")
	}
}

impl NumberSlot {
	pub fn less_than(self, other: Self) -> Self {
		unimplemented!("lt number")
	}
}

impl PartialEq for NumberSlot {
	fn eq(&self, other: &Self) -> bool {
		*self == *other
	}
}