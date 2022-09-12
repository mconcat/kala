use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign, Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};
use crate::bigint_decimal::Decimal;

pub trait JSNumber: Add + AddAssign + Div + DivAssign + Mul + MulAssign + Sub + SubAssign + Rem + RemAssign + Not + Ord + PartialOrd + Eq + PartialEq + Eq + Clone + Copy + From<i32> + Into<i32> {

}

#[derive(Debug, Clone, Copy)]
pub enum SMINumber {
    SMI(i32),
    Float(Decimal), // TODO: move to ieee_decimal::Decimal
}

impl SMINumber { 
    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Self::SMI(i) => i as i32, // TODO
            _ => 0,
        }
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        match self {
            &Self::SMI(i) => Some(i as i64),
            _ => None,
        }
    }
}

impl Add for SMINumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => Self::SMI(i + j),
            (Self::SMI(i), Self::Float(j)) => Self::Float(i.into() + j),
            (Self::Float(i), Self::SMI(j)) => Self::Float(i + j.into()),
            (Self::Float(i), Self::Float(j)) => Self::Float(i + j),
        }
    }    
}

impl Sub for SMINumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => Self::SMI(i - j),
            (Self::SMI(i), Self::Float(j)) => Self::Float(i.into() - j),
            (Self::Float(i), Self::SMI(j)) => Self::Float(i - j.into()),
            (Self::Float(i), Self::Float(j)) => Self::Float(i - j),
        }
    }    
}

impl Mul for SMINumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => Self::SMI(i * j),
            (Self::SMI(i), Self::Float(j)) => Self::Float(i.into() * j),
            (Self::Float(i), Self::SMI(j)) => Self::Float(i * j.into()),
            (Self::Float(i), Self::Float(j)) => Self::Float(i * j),
        }
    }    
}

impl Div for SMINumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => Self::Float(i / j),
            (Self::SMI(i), Self::Float(j)) => Self::Float(i.into() / j),
            (Self::Float(i), Self::SMI(j)) => Self::Float(i / j.into()),
            (Self::Float(i), Self::Float(j)) => Self::Float(i / j),
        }
    }    
}

impl Rem for SMINumber {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => Self::SMI(i % j),
            (Self::SMI(i), Self::Float(j)) => Self::Float(i.into() % j),
            (Self::Float(i), Self::SMI(j)) => Self::Float(i % j.into()),
            (Self::Float(i), Self::Float(j)) => Self::Float(i % j),
        }
    }    
}

impl AddAssign for SMINumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for SMINumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for SMINumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for SMINumber {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl RemAssign for SMINumber {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl PartialOrd for SMINumber {
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SMI(i), Self::SMI(j)) => i >= j,
            (Self::SMI(i), Self::Float(j)) => i.into() >= j,
            (Self::Float(i), Self::SMI(j)) => i >= j.into(),
            (Self::Float(i), Self::Float(j)) => i >= j,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SMI(i), Self::SMI(j)) => i > j,
            (Self::SMI(i), Self::Float(j)) => i.into() > j,
            (Self::Float(i), Self::SMI(j)) => i > j.into(),
            (Self::Float(i), Self::Float(j)) => i > j,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SMI(i), Self::SMI(j)) => i <= j,
            (Self::SMI(i), Self::Float(j)) => i.into() <= j,
            (Self::Float(i), Self::SMI(j)) => i <= j.into(),
            (Self::Float(i), Self::Float(j)) => i <= j,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SMI(i), Self::SMI(j)) => i < j,
            (Self::SMI(i), Self::Float(j)) => i.into() < j,
            (Self::Float(i), Self::SMI(j)) => i < j.into(),
            (Self::Float(i), Self::Float(j)) => i < j,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::SMI(i), Self::SMI(j)) => i.partial_cmp(j),
            (Self::SMI(i), Self::Float(j)) => i.into().partial_cmp(j),
            (Self::Float(i), Self::SMI(j)) => i.partial_cmp(&j.into()),
            (Self::Float(i), Self::Float(j)) => i.partial_cmp(j),
        }
    }
}

impl Ord for SMINumber {}

impl PartialEq for SMINumber {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::SMI(i), Self::SMI(j)) => i == j,
            (Self::SMI(i), Self::Float(j)) => i.into() == j,
            (Self::Float(i), Self::SMI(j)) => i == j.into(),
            (Self::Float(i), Self::Float(j)) => i == j,
        }
    }
}

impl Eq for SMINumber {}

#[cfg(test)]
mod test_number {
    enum Operator {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
    }

    #[test]
    fn simple_test() {
        let test_arithmetic = |mut ix: i64, iy: i64, op: Operator| {
            let mut jsx = crate::number::JSNumber::SMI(ix);
            let jsy = crate::number::JSNumber::SMI(iy);
            assert_eq!(jsx.to_i64().unwrap(), ix);
            assert_eq!(jsy.to_i64().unwrap(), iy);
            match op {
                Operator::Add => {
                    ix += iy;
                    jsx.op_add(&jsy)
                }
                Operator::Sub => {
                    ix -= iy;
                    jsx.op_sub(&jsy)
                }
                Operator::Mul => {
                    ix *= iy;
                    jsx.op_mul(&jsy)
                }
                Operator::Div => {
                    ix /= iy;
                    jsx.op_div(&jsy)
                }
                Operator::Mod => {
                    ix %= iy;
                    jsx.op_mod(&jsy)
                }
                /*
                ast::BinaryOp::BitAnd => jsx.op_bitand(&jsy),
                ast::BinaryOp::BitOr => jsx.op_bitor(&jsy),
                ast::BinaryOp::BitXor => jsx.op_bitxor(&jsy),
                ast::BinaryOp::LShift => jsx.op_lshift(&jsy),
                ast::BinaryOp::RShift => jsx.op_rshift(&jsy),
                ast::BinaryOp::URShift => jsx.op_urshift(&jsy),
                */
                _ => unimplemented!("Invalid op"),
            };

            assert_eq!(jsx.to_i64().unwrap(), ix);
        };

        // copilot wrote, add more edge cases later
        test_arithmetic(1, 2, Operator::Add);
        test_arithmetic(1, 2, Operator::Sub);
        test_arithmetic(1, 2, Operator::Mul);
        test_arithmetic(1, 2, Operator::Div);
        test_arithmetic(1, 2, Operator::Mod);

        test_arithmetic(1, -2, Operator::Add);
        test_arithmetic(1, -2, Operator::Sub);
        test_arithmetic(1, -2, Operator::Mul);
        test_arithmetic(1, -2, Operator::Div);
        test_arithmetic(1, -2, Operator::Mod);

        test_arithmetic(-1, 2, Operator::Add);
        test_arithmetic(-1, 2, Operator::Sub);
        test_arithmetic(-1, 2, Operator::Mul);
        test_arithmetic(-1, 2, Operator::Div);
        test_arithmetic(-1, 2, Operator::Mod);

        test_arithmetic(-1, -2, Operator::Add);
        test_arithmetic(-1, -2, Operator::Sub);
        test_arithmetic(-1, -2, Operator::Mul);
        test_arithmetic(-1, -2, Operator::Div);
        test_arithmetic(-1, -2, Operator::Mod);

        test_arithmetic(0, 2, Operator::Add);
        test_arithmetic(0, 2, Operator::Sub);
        test_arithmetic(0, 2, Operator::Mul);
        test_arithmetic(0, 2, Operator::Div);
        test_arithmetic(0, 2, Operator::Mod);
    } 
}