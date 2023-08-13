use std::{ops::{Deref, Add, Sub, Mul, Div, Neg}};

use super::NumberSlot;

impl Add for NumberSlot {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if self.is_inline() && rhs.is_inline() {
			let (res, overflow) = self.value.overflowing_add(rhs.value);
			if overflow {
				return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value: i128 = self.into();
		let rhs_value: i128 = rhs.into();

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
		if self.is_inline() && rhs.is_inline() {
			let (res, overflow) = self.value.overflowing_sub(rhs.value);
			if overflow {
				//return NumberSlot::new(res as i128)
				unimplemented!("overflow")
			}
			return NumberSlot::new_inline(res) 
		}	

		let self_value: i128 = self.into();
		let rhs_value: i128 = rhs.into();
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
		if self.is_inline() && rhs.is_inline() {
			let (res, overflow) = self.value.overflowing_mul(rhs.value);
			if overflow {
				unimplemented!("overflow")
				//return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value: i128 = self.into();
		let rhs_value: i128 = rhs.into();

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
		if self.is_inline() && rhs.is_inline() {
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

		unimplemented!("long division")
	}
}

impl Neg for NumberSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.is_inline() {
            return NumberSlot::new_inline(-self.value)
        }

        let value: i128 = self.into();
        NumberSlot::new(-value)
    }
}