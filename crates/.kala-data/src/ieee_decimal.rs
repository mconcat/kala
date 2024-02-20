/// low-level numeric data type

use derive_more::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign,
    Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign
};

use std::ops::{Add, Div, Mul, Sub, Rem, Neg};

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
    const BIAS: u64 = 255;

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

    pub fn is_negative(&self) -> bool {
        self.sign()
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

    pub fn from_parts(sign: bool, biased_exponent: u64, mantissa: u64, number_class: NumberClass) -> Self {
        let mut result = 0;

        match number_class {
            NumberClass::PositiveZero | NumberClass::NegativeZero => {
                // No need to set exponent and mantissa bits, as they're zero for zero
                result |= (sign as u64) << (Self::K - 1);
            },
            NumberClass::PositiveNormal 
            | NumberClass::NegativeNormal
            | NumberClass::PositiveSubnormal
            | NumberClass::NegativeSubnormal => {
                result |= (biased_exponent << Self::mantissa_mask_direct_length()) & Self::exponent_mask_direct();
                result |= mantissa & Self::mantissa_mask_direct();
            },
            NumberClass::PositiveInfinity | NumberClass::NegativeInfinity => {
                // No need to set the mantissa bits, as it's zero for infinity
                result |= Self::exponent_mask_direct();
            },
            NumberClass::QuietNaN | NumberClass::SignalingNaN => {
                result |= Self::exponent_mask_direct();
                result |= Self::mantissa_mask_direct();

                // in the case of NaN, set the mantissa to something other than zero
                result != mantissa & Self::mantissa_mask_direct();
                result |= (sign as u64) << (Self::K - 1);
            }
        }

        Self(result)
    }

    /// Creates a quiet NaN value.
    /// 
    /// A quiet NaN is a NaN value that can be used in arithmetic operations without causing an exception.
    /// 
    /// To create a quiet NaN, the exponent field is set to all 1s and the mantissa field is set to any non-zero value, but the sign bit is set to 0.
    /// In this implementation, the mantissa field is set to have the most significant bit set(MSB) to 1.
    /// 
    /// sign bit   set to Non-zero value
    /// v          v
    /// 0111 1111  1000 0000 0000 0000 0000 0000
    /// |-------| |----------------------------|
    ///  exponent            mantissa
    pub fn quiet_nan() -> Self {
        // In IEEE 754, NaN's sign bit is always set to 0.
        let sign = false;

        // All bits set to 1
        let biased_exponent = u64::MAX;

        // MSB set to 1
        let mantissa = 1 << (Self::mantissa_mask_direct_length() - 1);

        let number_class = NumberClass::QuietNaN;

        Self::from_parts(sign, biased_exponent, mantissa, number_class)
    }

    /// In IEEE 754, a infinity value is represented by setting the exponent field to all 1s and the mantissa field to all 0s.
    /// sign bit is set to 0 for positive infinity and 1 for negative infinity.
    /// 
    /// sign bit
    /// v
    /// 0111 1111  0000 0000 0000 0000 0000 0000
    /// |-------| |----------------------------|
    ///  exponent            mantissa
    pub fn positive_infinity() -> Self {
        // set sign bit to 0
        let sign = false;
        let biased_exponent = u64::MAX;

        // all bits set to 0
        let mantissa = 0;
        let number_class = NumberClass::PositiveInfinity;

        Self::from_parts(sign, biased_exponent, mantissa, number_class)
    }

    /// sign bit
    /// v
    /// 1111 1111  0000 0000 0000 0000 0000 0000
    /// |-------| |----------------------------|
    ///  exponent            mantissa
    pub fn negative_infinity() -> Self {
        // set sign bit to 1
        let sign = true;

        let biased_exponent = u64::MAX;
        let mantissa = 0;
        let number_class = NumberClass::NegativeInfinity;

        Self::from_parts(sign, biased_exponent, mantissa, number_class)
    }

    /// In IEEE 754, a zero value is represented by setting the both exponent and mantissa field to all 0s.
    /// 
    /// The sign bit is used to distinguish between positive and negative zero.
    /// if sign bit is set to 0, then the value is positive zero, otherwise it's negative zero.
    /// 
    /// sign bit
    /// v
    /// 0000 0000  0000 0000 0000 0000 0000 0000
    /// |-------| |----------------------------|
    ///  exponent           mantissa
    pub fn positive_zero() -> Self {
        let sign = false;
        let biased_exponent = 0;
        let mantissa = 0;
        let number_class = NumberClass::PositiveZero;

        Self::from_parts(sign, biased_exponent, mantissa, number_class)
    }

    /// sign bit
    /// v
    /// 1000 0000  0000 0000 0000 0000 0000 0000
    /// |-------| |----------------------------|
    ///  exponent          mantissa
    pub fn negative_zero() -> Self {
        let sign = true;
        let biased_exponent = 0;
        let mantissa = 0;
        let number_class = NumberClass::NegativeZero;

        Self::from_parts(sign, biased_exponent, mantissa, number_class)
    }

    // TODO: this implementation is very clumsy, need to be improved
    pub fn divide_by_tens_power(mantissa: u64, shift: u64) -> u64 {
        let result = mantissa as f64 / 10.0_f64.powf(shift as f64);

        // Round the result to the nearest integer
        if result.fract() >= 0.5 {
            return result.ceil() as u64;
        }

        result.floor() as u64
    }

    /// The `normalize` function normalizes the mantissa of a decimal number.
    /// 
    /// term **normalization** is the process of adjusting the mantissa of a decimal number to make it as small as possible.
    /// 
    /// In this implementation, the function first extracts the `biased_exponent` and `mantissa`. 
    /// It then shifts the `mantissa` left by one bit and decreases the `biased_exponent` by one until the mantissa is less than `0x10000000000000`. 
    /// and then returns the `biased_exponent` and `mantissa`.
    /// 
    /// Why should biased_exponent should be decreased by one until the mantissa is less than `0x10000000000000`?
    /// 
    /// Because shifting the mantissa left makes the fractional part smaller 
    /// and decreasing the exponent moves the value of the floating-point number to the left.
    pub fn normalize(biased_exponent: u64, mantissa: u64) -> (u64, u64) {
        let mut biased_exponent = biased_exponent;
        let mut mantissa = mantissa;

        while mantissa < 0x10000000000000 {
            mantissa <<= 1;
            biased_exponent -= 1;
        }

        (biased_exponent, mantissa)
    }

    /// Rounds the current Decimal64 to the nearest even number.
    pub fn round(&self, mantissa: u64) -> Self {
        let (mut biased_exponent, _, number_class) = Self::into_parts(&self);

        // for special cases, return the same value
        match number_class {
            NumberClass::SignalingNaN
            | NumberClass::QuietNaN
            | NumberClass::NegativeInfinity
            | NumberClass::PositiveInfinity
            | NumberClass::NegativeZero
            | NumberClass::PositiveZero => return *self,
            _ => {}
        }

        // If the biased exponent is 0, this means the Decimal64 is either a subnormal number or zero.
        // In this case, the function also simply returns the original Decimal64.
        if biased_exponent == 0 {
            return *self;
        }

        let frac_part = mantissa as f64 / 10.0_f64.powf(Self::T as f64);
        let integer_part = mantissa as f64 / 10.0_f64.powf(Self::T as f64);

        let mut rounded_mantissa = if frac_part > 0.5 {
            integer_part.ceil()
        } else if frac_part < 0.5 {
            integer_part.floor()
        } else {
            // If the fractional part is exactly 0.5, we apply "round half to even(banker's rounding)" mode.
            // This means that we round to the nearest even number. 
            if integer_part % 2.0 == 0.0 {
                integer_part
            } else {
                integer_part + 1.0
            }
        };

        // if rounding up caused the mantissa to increase its length,
        // adjust the exponent and mantissa accordingly.
        if rounded_mantissa >= 10.0_f64.powf(Self::T as f64) {
            rounded_mantissa /= 10.0_f64.powf(Self::T as f64);
            biased_exponent += 1;
        }

        let rounded_mantissa = Self::divide_by_tens_power(rounded_mantissa as u64, Self::T);

        Self::from_parts(self.sign(), biased_exponent, rounded_mantissa, number_class)
    }

    // This function converts a f64 value to a Decimal64 value.
    // 
    // The f64 value is represented in binary, with the sign bit in the most significant bit. 
    // the exponent bits in the next 11 bits, and the mantissa bits in the remaining 52 bits.
    // 
    // Decimal64 value is represented in binary, with the sign bit in the most significant bit, 
    // the biased exponent bits in the next 11 bits, and the mantissa bits in the remaining 52 bits.
    // 
    // The biased exponent bits are the exponent bits plus 1023.
    // 
    // The mantissa bits are the mantissa bits plus 0x10000000000000.
    // 
    // The Decimal64 value is created by combining the sign bit, the biased exponent bits, and the mantissa bits.
    //
    // pub fn from_f64(value: f64) -> Self {
    //     let bits = value.to_bits();
    //     let sign = bits >> 63 != 0;
    //     let exponent = ((bits >> 52) & 0x7ff) as i32;
    //     let mantissa = bits & 0xfffffffffffff;

    //     let biased_exponent = if exponent == 0 {
    //         0
    //     } else {
    //         (exponent as u64) + 1023
    //     };

    //     let mantissa = if exponent == 0 {
    //         mantissa
    //     } else {
    //         mantissa | 0x10000000000000
    //     };

    //     Self::from_parts(sign, biased_exponent, mantissa, NumberClass::PositiveNormal)
    // }
}

impl Add<Self> for Decimal64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {        
        let (a_biased_exponent, mut a_mantissa, a_class) = self.into_parts();
        let (b_biased_exponent, mut b_mantissa, b_class) = rhs.into_parts();

        let sign = self.is_negative() ^ rhs.is_negative();

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

            (NumberClass::PositiveZero, _) | (NumberClass::NegativeZero, _) => rhs,
            (_, NumberClass::PositiveZero) | (_, NumberClass::NegativeZero) => self,

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
                let round_decimal = self.round(normalized_mantissa);

                let (_, rounded_mantissa, _) = round_decimal.into_parts();

                // 5. construct the result
                Self::from_parts(sign, normalized_exponent, rounded_mantissa, NumberClass::PositiveNormal)
            }
        }
    }
}

impl Neg for Decimal64 {
    type Output = Decimal64;

    fn neg(self) -> Self {
        if self.is_exotic() {
            return self;
        }

        let mut result = self;
        result.0 ^= 1 << (Self::K - 1);
        
        result
    }
}

impl Sub<Self> for Decimal64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl Mul<Self> for Decimal64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let (a_biased_exponent, a_mantissa, a_class) = self.into_parts();
        let (b_biased_exponent, b_mantissa, b_class) = rhs.into_parts();
        let sign = self.is_negative() ^ rhs.is_negative();

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
                if rhs.is_negative() {
                    return Self::negative_infinity();
                }

                Self::positive_infinity()
            },
            (NumberClass::NegativeInfinity, x) |
            (x, NumberClass::NegativeInfinity) => {
                if rhs.is_negative() {
                    return Self::positive_infinity();
                }

                Self::negative_infinity()
            },

            (NumberClass::NegativeZero, NumberClass::NegativeZero) |
            (NumberClass::PositiveZero, NumberClass::PositiveZero) => Self::positive_zero(),
            (NumberClass::NegativeZero, NumberClass::PositiveZero) |
            (NumberClass::PositiveZero, NumberClass::NegativeZero) => Self::negative_zero(),

            (NumberClass::PositiveZero, x) |
            (x, NumberClass::PositiveZero) => {
                if rhs.is_negative() {
                    return Self::negative_zero();
                }

                Self::positive_zero()
            },

            (NumberClass::NegativeZero, x) |
            (x, NumberClass::NegativeZero) => {
                if rhs.is_negative() {
                    return Self::positive_zero();
                }

                Self::negative_zero()
            },

            // at this point, we know that both numbers are normal or subnormal
            (_, _) => {
                // 1. Add the exponents of the two numbers
                let denormal_exponent = a_biased_exponent + b_biased_exponent - Self::BIAS;
            
                // 2. Multiply the mantissas of the two numbers
                let denormal_mantissa = a_mantissa as u128 * b_mantissa as u128;
                
                // 3. Normalize the result
                let (normalized_exponent, normalized_mantissa) = Self::normalize(denormal_exponent, denormal_mantissa as u64);

                // 4. Round the result
                let round_decimal = self.round(normalized_mantissa);
                let (_, rounded_mantissa, _) = round_decimal.into_parts();

                // 5. Construct the result
                Self::from_parts(sign, normalized_exponent, rounded_mantissa, NumberClass::PositiveNormal)
            }
        } 
    }
}

impl Div<Self> for Decimal64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let (a_biased_exponent, mut a_mantissa, a_class) = self.into_parts();
        let (b_biased_exponent, mut b_mantissa, b_class) = rhs.into_parts();

        let sign = self.is_negative() ^ rhs.is_negative();

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

            (NumberClass::PositiveZero, _) | (NumberClass::NegativeZero, _) => rhs,
            (_, NumberClass::PositiveZero) | (_, NumberClass::NegativeZero) => self,

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

                // 2. divide the mantissa of the larger number by the mantissa of the smaller number
                let denormal_mantissa = a_mantissa / b_mantissa;

                // 3. normalize the result
                let (normalized_exponent, normalized_mantissa) = Self::normalize(denormal_exponent, denormal_mantissa);

                // 4. round the result
                let round_decimal = self.round(normalized_mantissa);
                let (_, rounded_mantissa, _) = round_decimal.into_parts();

                // 5. construct the result
                Self::from_parts(sign, normalized_exponent, rounded_mantissa, NumberClass::PositiveNormal)
            }
        }
    }
}