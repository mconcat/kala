use std::{ops::{Deref, Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr}, mem::uninitialized};

use super::F128;

impl F128 {
	pub fn new_positive_infinity() -> Self {
		Self(i128::MAX)
	}

	pub fn new_negative_infinity() -> Self {
		Self(i128::MIN)
	}
}

impl Add for F128 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		// TODO: check if already infinity

		let (res, overflow) = self.0.overflowing_add(rhs.0);
		if overflow {
			if res > 0 {
				return Self::new_negative_infinity()
			} else {
				return Self::new_positive_infinity()
			}
		}

		Self(res)
	}
}
/* 
impl Add for &F128 {
	type Output = F128;

	fn add(self, rhs: Self) -> Self::Output {
		*self + *rhs
	}
}
*/

impl Sub for F128 {
	type Output = Self; 

	fn sub(self, rhs: Self) -> Self::Output {
		// TODO: check if already infinity

		let (res, overflow) = self.0.overflowing_sub(rhs.0);
		if overflow {
			if res > 0 {
				return Self::new_negative_infinity()
			} else {
				return Self::new_positive_infinity()
			}
		}

		Self(res)
	}
}

impl Mul for F128 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		// TODO: check if already infinity

		let i0 = self.0 >> 64;
		let f0 = self.0 & 0xFFFFFFFFFFFFFFFF;
		let i1 = rhs.0 >> 64;
		let f1 = rhs.0 & 0xFFFFFFFFFFFFFFFF;

		// fast path
		if f0 == 0 && f1 == 0 {
			let i = i0 * i1;
			if i >= i128::from(i64::MAX) {
				return Self::new_positive_infinity()
			}
			if i <= i128::from(i64::MIN) {
				return Self::new_negative_infinity()
			}
			return Self(i << 64)
		}

		unimplemented!("number mul")
	}
}

impl Div for F128 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		unimplemented!("number div")
	}
}

impl Neg for F128 {
    type Output = Self;

    fn neg(self) -> Self::Output {
		Self(-self.0)
	}
}

impl BitAnd for F128 {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitand")
	}
}

impl BitOr for F128 {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitor")
	}
}

impl BitXor for F128 {
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self::Output {
		unimplemented!("number bitxor")
	}
}

impl Shl for F128 {
	type Output = Self;

	fn shl(self, rhs: Self) -> Self::Output {
		unimplemented!("number shl")
	}
}

impl Shr for F128 {
	type Output = Self;

	fn shr(self, rhs: Self) -> Self::Output {
		unimplemented!("number shr")
	}
}

impl F128 {
	pub fn less_than(self, other: Self) -> Self {
		unimplemented!("lt number")
	}
}

impl PartialEq for F128 {
	fn eq(&self, other: &Self) -> bool {
		*self == *other
	}
}