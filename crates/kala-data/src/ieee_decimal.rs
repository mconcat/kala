/// low-level numeric data type

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign,
    Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign
};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign, Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

pub trait Number: 
    Clone +
    Eq +
    Copy +
    PartialOrd +
    Ord +
    Add<Self, Output=Self> +
    Sub<Self, Output=Self> +
    Mul<Self, Output=Self> +
    Div<Self, Output=Self> +
    Rem<Self, Output=Self> 
{
}

pub enum NumberClass {
    SignalingNaN,
    QuietNaN,
    NegativeInfinity,
    NegativeNormal,
    NegativeSubnormal,
    NegativeZero,
    PositiveZero,
    PositiveSubnormal,
    PositiveNormal,
    PositiveInfinity,
}

#[derive(
    Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Debug, 
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign
)]
pub struct SMI(i32);

#[derive(
    Clone, Eq, Copy, PartialEq, PartialOrd, Ord, Debug
)]
pub struct Decimal64(u64);

// https://irem.univ-reunion.fr/IMG/pdf/ieee-754-2008.pdf#page25
impl Decimal64 {
    const K: u64 = 64;
    const W: u64 = 8;
    const C: u64 = Self::W+5;
    const T: u64 = 50;

    // [S, G_0, G_1, .. G_(w+5), T_0, T_1, .. T_n]

    // S: sign bit
    const fn sign_mask() -> u64 {
        1 << (Self::K - 1)
    }

    #[inline]
    pub fn sign(&self) -> bool {
        self.0 & Self::sign_mask() != 0
    }

    // [G_0, G_1]: combination bits
    const fn combination_mask() -> u64 {
        0b11111 << (Self::K - 5)
    }

    #[inline]
    pub fn combination(&self) -> u64 {
        (self.0 & Self::combination_mask()) >> (Self::K - 5)
    }

    //     |------------(w+2)--------------|  |----------------------(C-(w+2)+T)-------------------------|
    // [S, exponent: [G_0, G_1, .. G_(w+1)], mantissa: [G_(w+2), G_(w+3), G_(w+4), T_0, T_1, .. T_n]] - direct
    const fn mantissa_mask_direct_length() -> u64 {
        Self::C - (Self::W + 2) + Self::T
    }

    const fn mantissa_mask_direct() -> u64 {
        (1 << Self::mantissa_mask_direct_length()) - 1
    }

    #[inline]
    fn mantissa_direct(&self) -> u64 {
        self.0 & Self::mantissa_mask_direct()
    }

    const fn exponent_mask_length() -> u64 {
        Self::W + 2
    }

    const fn exponent_mask_direct() -> u64 {
        ((1 << Self::exponent_mask_length()) - 1) << Self::mantissa_mask_direct_length()
    }

    #[inline]
    fn exponent_direct(&self) -> u64 {
        (self.0 & Self::exponent_mask_direct()) >> Self::mantissa_mask_direct_length()
    }
    
    //           |------------(w+2)-------------|  |-----------------(1+T)----------------|
    // [S, 1, 1, exponent: [G_2, G_3, .. G_(w+3)], mantissa: [G_(w+4), T_0, T_1, .. T_n]] - extended
    const fn mantissa_mask_extended_length() -> u64 {
        1 + Self::T
    }

    const fn mantissa_mask_extended() -> u64 {
        (1 << Self::mantissa_mask_extended_length()) - 1
    }

    #[inline]
    fn mantissa_extended(&self) -> u64 {
        self.0 & Self::mantissa_mask_extended()
    }

    const fn exponent_mask_extended() -> u64 {
        ((1 << Self::exponent_mask_length()) - 1) << Self::mantissa_mask_extended_length()
    }

    #[inline]
    fn exponent_extended(&self) -> u64 {
        (8 << Self::T) | ((self.0 & Self::exponent_mask_extended()) >> Self::mantissa_mask_extended_length())
    }

    // [G_4, G_5]
    const fn exotic_mask() -> u64 {
        11 << (Self::K - 7)
    }

    #[inline]
    fn exotic_class(&self) -> NumberClass {
        match self.0 & Self::exotic_mask() {
            0b00 => match self.sign() {
                true => NumberClass::NegativeInfinity,
                false => NumberClass::PositiveInfinity,
            },
            0b01 => unreachable!("not a canonical representation of infinity"),
            0b10 => NumberClass::QuietNaN,
            0b11 => NumberClass::SignalingNaN,
        }
    } 

    #[inline]
    fn nan_payload(&self) -> u64 {
        self.0 & (1 << (Self::T)) - 1
    }

    pub fn is_exotic(&self) -> bool {
        self.combination() == 0b1111
    }

    // into_parts() returns the raw bits representation of the decimal,
    // consists of biased exponent and mantissa
    pub fn into_parts(&self) -> (u64, u64, NumberClass) {
        let sign = self.sign();

        match self.combination() {
            0b0000 => {
                let biased_exponent = self.exponent_direct();
                let mantissa = self.mantissa_direct();
                let class = match biased_exponent {
                    0 => match mantissa {
                        0 => if sign { NumberClass::NegativeZero } else { NumberClass::PositiveZero },
                        _ => if sign { NumberClass::NegativeSubnormal } else { NumberClass::PositiveSubnormal },
                    },
                    _ => if sign { NumberClass::NegativeNormal } else { NumberClass::PositiveNormal },
                };
                (biased_exponent, mantissa, class)
            },
            0b0001 |
            0b0010 |
            0b0011 |
            0b0100 |
            0b0101 |
            0b0110 |
            0b0111 |
            0b1000 |
            0b1001 |
            0b1010 |
            0b1011 => {
                let biased_exponent = self.exponent_direct();
                let mantissa = self.mantissa_direct();
                (biased_exponent, mantissa, if sign { NumberClass::NegativeNormal } else { NumberClass::PositiveNormal })
            },
            0b1100 |
            0b1101 |
            0b1110 => {
                let biased_exponent = self.exponent_extended();
                let mantissa = self.mantissa_extended();
                let class = match biased_exponent {
                    0 => if sign { NumberClass::NegativeSubnormal } else { NumberClass::PositiveSubnormal },
                    _ => if sign { NumberClass::NegativeNormal } else { NumberClass::PositiveNormal },
                };
                (biased_exponent, mantissa, class)
            }
            0b1111 => (0, 0, self.exotic_class())
        }
    }
}

impl Add<Self> for Decimal64 {
    fn add(self, other: Self) -> Self {
        let (a, b) = if self >= other {
            (self, other)
        } else {
            (other, self)
        };        

        let (a_biased_exponent, a_mantissa, a_class) = a.into_parts();
        let (b_biased_exponent, b_mantissa, b_class) = b.into_parts();

        match (a_class, b_class) {
            (NumberClass::SignalingNaN, _) | (_, NumberClass::SignalingNaN) => Self::quiet_nan(),
            (NumberClass::QuietNaN, _) | (_, NumberClass::QuietNaN) => Self::quiet_nan(),
            (NumberClass::NegativeInfinity, NumberClass::PositiveInfinity) | 
            (NumberClass::PositiveInfinity, NumberClass::NegativeInfinity) => Self::quiet_nan(),
            (NumberClass::NegativeInfinity, _) | (_, NumberClass::NegativeInfinity) => Self::negative_infinity(),
            (NumberClass::PositiveInfinity, _) | (_, NumberClass::PositiveInfinity) => Self::positive_infinity(),

            (NumberClass::NegativeZero, NumberClass::NegativeZero) => Self::negative_zero(),
            (NumberClass::PositiveZero, NumberClass::PositiveZero) |
            (NumberClass::NegativeZero, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::NegativeZero) => Self::positive_zero(),

            (NumberClass::PositiveZero, _) | (NumberClass::NegativeZero, _) => b,
            (_, NumberClass::PositiveZero) | (_, NumberClass::NegativeZero) => a,

            // at this point, we know that both numbers are normal or subnormal
            (_, _) => {
                // 1. divide the mantissa of the smaller number to align the decimal point
                let denormal_exponent = if a_biased_exponent > b_biased_exponent {
                    let shift = a_biased_exponent - b_biased_exponent;
                    b_mantissa = Self::divide_by_tens_power(b_mantissa, shift);
                    a_biased_exponent
                } else {
                    let shift = b_biased_exponent - a_biased_exponent;
                    a_mantissa = Self::divide_by_tens_power(a_mantissa, shift);
                    b_biased_exponent
                };

                // 2. add the mantissa of the smaller number to the mantissa of the larger number
                let denormal_mantissa = a_mantissa + b_mantissa;

                // 3. normalize the result
                let (normalized_exponent, normalized_mantissa) = Self::normalize(denormal_exponent, denormal_mantissa);

                // 4. round the result
                let round_mantissa = Self::round(normalized_mantissa);

                // 5. construct the result
                Self::from_parts(normalized_exponent, round_mantissa)
            }
        }
    }
}

impl Neg<Self> for Decimal64 {
    fn neg(mut self) -> Self {
        if self.is_exotic() {
            return self;
        }

        self.0 ^= 1 << (Self::K - 1);
        self
    }
}

impl Sub<Self> for Decimal64 {
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl Mul<Self> for Decimal64 {
    fn mul(self, other: Self) -> Self {
        let (a, b) = if self >= other {
            (self, other)
        } else {
            (other, self)
        };        

        let (a_biased_exponent, a_mantissa, a_class) = a.into_parts();
        let (b_biased_exponent, b_mantissa, b_class) = b.into_parts();

        match (a_class, b_class) {
            (NumberClass::SignalingNaN, _) | (_, NumberClass::SignalingNaN) => Self::quiet_nan(),
            (NumberClass::QuietNaN, _) | (_, NumberClass::QuietNaN) => Self::quiet_nan(),

            (NumberClass::PositiveInfinity, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::PositiveInfinity) |
            (NumberClass::PositiveInfinity, NumberClass::NegativeZero) |
            (NumberClass::NegativeZero, NumberClass::PositiveInfinity) |
            (NumberClass::NegativeInfinity, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::NegativeInfinity) |
            (NumberClass::NegativeInfinity, NumberClass::NegativeZero) |
            (NumberClass::NegativeZero, NumberClass::NegativeInfinity) => Self::quiet_nan(),

            (NumberClass::PositiveInfinity, x) |
            (x, NumberClass::PositiveInfinity) => {
                if b.is_negative() {
                    Self::negative_infinity()
                } else {
                    Self::positive_infinity()
                }
            },
            (NumberClass::NegativeInfinity, x) |
            (x, NumberClass::NegativeInfinity) => {
                if b.is_negative() {
                    Self::positive_infinity()
                } else {
                    Self::negative_infinity()
                }
            },

            (NumberClass::NegativeZero, NumberClass::NegativeZero) |
            (NumberClass::PositiveZero, NumberClass::PositiveZero) => Self::positive_zero(),
            (NumberClass::NegativeZero, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::NegativeZero) => Self::negative_zero(),

            (NumberClass::PositiveZero, x) |
            (x, NumberClass::PositiveZero) => {
                if b.is_negative() {
                    Self::negative_zero()
                } else {
                    Self::positive_zero()
                }
            },

            (NumberClass::NegativeZero, x) |
            (x, NumberClass::NegativeZero) => {
                if b.is_negative() {
                    Self::positive_zero()
                } else {
                    Self::negative_zero()
                }
            },

            // at this point, we know that both numbers are normal or subnormal
            (_, _) => {
                // 1. Add the exponents of the two numbers
                let denormal_exponent = a_biased_exponent + b_biased_exponent - 255; // TODO: parameterize
            
                // 2. Multiply the mantissas of the two numbers
                let denormal_mantissa = (a_mantissa as u128 * b_mantissa as u128);
                
            }
        } 
    }
}

impl Div<Self> for Decimal64 {
    fn div(self, other: Self) -> Self {
        let (a, b) = if self >= other {
            (self, other)
        } else {
            (other, self)
        };        

        let (a_biased_exponent, a_mantissa, a_class) = a.into_parts();
        let (b_biased_exponent, b_mantissa, b_class) = b.into_parts();

        match (a_class, b_class) {
            (NumberClass::SignalingNaN, _) | (_, NumberClass::SignalingNaN) => Self::quiet_nan(),
            (NumberClass::QuietNaN, _) | (_, NumberClass::QuietNaN) => Self::quiet_nan(),
            (NumberClass::NegativeInfinity, NumberClass::PositiveInfinity) | 
            (NumberClass::PositiveInfinity, NumberClass::NegativeInfinity) => Self::quiet_nan(),
            (NumberClass::NegativeInfinity, _) | (_, NumberClass::NegativeInfinity) => Self::negative_infinity(),
            (NumberClass::PositiveInfinity, _) | (_, NumberClass::PositiveInfinity) => Self::positive_infinity(),

            (NumberClass::NegativeZero, NumberClass::NegativeZero) => Self::negative_zero(),
            (NumberClass::PositiveZero, NumberClass::PositiveZero) |
            (NumberClass::NegativeZero, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::NegativeZero) => Self::positive_zero(),

            (NumberClass::PositiveZero, _) | (NumberClass::NegativeZero, _) => b,
            (_, NumberClass::PositiveZero) | (_, NumberClass::NegativeZero) => a,

            // at this point, we know that both numbers are normal or subnormal
            (_, _) => {
                // 1. divide the mantissa of the smaller number to align the decimal point
                let denormal_exponent = if a_biased_exponent > b_biased_exponent {
                    let shift = a_biased_exponent - b_biased_exponent;
                    b_mantissa = Self::divide_by_tens_power(b_mantissa, shift);
                    a_biased_exponent
                } else {
                    let shift = b_biased_exponent - a_biased_exponent;
                    a_mantissa = Self::divide_by_tens_power(a_mantissa, shift);
                    b_biased_exponent
                };
            }
        }
    }
}