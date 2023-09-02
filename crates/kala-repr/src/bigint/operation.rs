use std::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr};

use super::BigintSlot;

impl Add for BigintSlot {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint add")
    }
}

impl Sub for BigintSlot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint sub")
    }
}

impl Mul for BigintSlot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint mul")
    }
}

impl Div for BigintSlot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint div")
    }
}

impl Neg for BigintSlot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unimplemented!("bigint neg")
    }
}

impl BitAnd for BigintSlot {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint bitand")
    }
}

impl BitOr for BigintSlot {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint bitor")
    }
}

impl BitXor for BigintSlot {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint bitxor")
    }
}

impl Shl for BigintSlot {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint shl")
    }
}

impl Shr for BigintSlot {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        unimplemented!("bigint shr")
    }
}

impl BigintSlot {
    pub fn less_than(self, other: Self) -> Self {
        unimplemented!("bigint lt")
    }
}

/* 
impl Add for BigintSlot {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_inline() && rhs.is_inline() {
			let res = (self.sign_len as i64) + (rhs.sign_len as i64);
			if res < i32::MIN.into() || res > i32::MAX.into() {
                unimplemented!("asdf")
				// return BigintSlot::new(res as i128)
			}
			return BigintSlot::new_inline(res as i32) 
		}

        let (self_sign, mut self_value) = (&self).into();
        let (rhs_sign, mut rhs_value) = (&rhs).into();

        if self_value.len() > rhs_value.len() {
            (self_value, rhs_value) = (rhs_value, self_value)
        }

        let mut overflow = false;
        let mut res = BigintSlot::with_capacity(rhs_value.len());

        for (i, digit) in self_value.into_iter().enumerate() {
            let rhs_digit = rhs_value[i];

            (res.pointer[i as u32], overflow) = digit.carrying_add(rhs_digit, overflow);
        }

        res.sign_len = rhs.sign_len; // XXXXXXX

        res
    }
}
*/

impl PartialEq for BigintSlot {
    fn eq(&self, other: &Self) -> bool {
        if self.is_inline() && other.is_inline() {
            return self.sign_len == other.sign_len;
        }

        unimplemented!("asdf")

        /* 
        let (self_sign, self_value) = self.into();
        let (other_sign, other_value) = other.into();

        if self_sign != other_sign {
            return false
        }

        if self_value.len() != other_value.len() {
            return false
        }

        for (i, digit) in self_value.into_iter().enumerate() {
            if digit != other_value[i] {
                return false
            }
        }

        true
        */
    }
}