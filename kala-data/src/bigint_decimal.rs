// implementation of decimal arithmetic on uint128

use core::panic;
use std::iter::zip;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign, Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decimal {
    Number(i128),
    Infinity(bool),
    NaN,
}

const PRECISION: i128 = 10**18;

impl Decimal {
    fn is_positive(&self) -> bool {
        match self {
            Decimal::Number(n) => *n >= 0,
            Decimal::Infinity(n) => !n,
            Decimal::NaN => false,
        }
    }

    fn positive_infinity() -> Self {
        Decimal::Infinity(false)
    }

    fn negative_infinity() -> Self {
        Decimal::Infinity(true)
    }

    fn number(n: i128) -> Self {
        Decimal::Number(n)
    }
}

impl Neg for Decimal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Decimal::Number(n) => Decimal::Number(-n),
            Decimal::Infinity(n) => Decimal::Infinity(!n),
            Decimal::NaN => Decimal::NaN,
        }
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::NaN, _) | (_, Decimal::NaN) => Decimal::NaN,
            (Decimal::Infinity(n), Decimal::Infinity(m)) => {
                if n == m {
                    Decimal::Infinity(n)
                } else {
                    Decimal::NaN
                }
            },
            (Decimal::Infinity(_), _) => self,
            (_, Decimal::Infinity(_)) => rhs,
            (Decimal::Number(i), Decimal::Number(j)) => {
                let (result, overflow) = i.overflowing_add(j);
                if overflow {
                    if result >= 0 { // underflow
                        Decimal::negative_infinity()
                    } else { // overflow
                        Decimal::positive_infinity()
                    }
                } else {
                    Decimal::number(result)
                }
            },
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs.neg())
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::NaN, _) | (_, Decimal::NaN) => Decimal::NaN,
            (Decimal::Infinity(n), Decimal::Infinity(m)) => Decimal::Infinity(n != m),
            (Decimal::Infinity(n), m) => Decimal::Infinity(n != m.is_positive()),
            (n, Decimal::Infinity(m)) => Decimal::Infinity(m != n.is_positive()),
            (Decimal::Number(i), Decimal::Number(j)) => {
                let result_is_negative = i.is_negative() != j.is_negative();
                let (i, j) = (i.abs().to_le(), j.abs().to_le());
                let [i_lo_64, i_hi_64]: [u64; 2] = unsafe { transmute(i) };
                let [j_lo_64, j_hi_64]: [u64; 2] = unsafe { transmute(j) };

                let mut result = [0u64; 4];

                // [mul_lo_lo, mul_lo_hi(w/carry_lo_lo), carry_lo_hi, 0]

                let (mul_lo_lo, carry_lo_lo) = i_lo_64.carrying_mul(j_lo_64, 0); // offset 0
                let (mul_lo_hi, carry_lo_hi) = i_lo_64.carrying_mul(j_hi_64, carry_lo_lo); // offset 1

                // [0, mul_hi_lo, mul_hi_hi(w/carry_hi_lo), carry_hi_hi]

                let (mul_hi_lo, carry_hi_lo) = i_hi_64.carrying_mul(j_lo_64, 0); // offset 1
                let (mul_hi_hi, carry_hi_hi) = i_hi_64.carrying_mul(j_hi_64, carry_hi_lo); // offset 2

                // [mul_lo_lo, mul_lo_hi+mul_hi_lo, mul_hi_hi+carry_lo_hi+carry, carry_hi_hi+carry]

                result[0] = mul_lo_lo;

                let (result_1, carry_1) = mul_lo_hi.carrying_add(mul_hi_lo, 0);
                result[1] = result_1;

                let (result_2, carry_2) = mul_hi_hi.carrying_add(carry_lo_hi, carry_1);
                result[2] = result_2;

                let (result_3, carry_3) = carry_hi_hi.overflowing_add(carry_2);
                if carry_3 { panic!("unexpected overflow over uint256 range") };
                result[3] = result_3;

                Self::normalize_256(result, result_is_negative)
            },
        }
    }
}

impl Div for Decimal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 * PRECISION / rhs.0)
    }
}

