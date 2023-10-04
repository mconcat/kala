use std::{ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Shl, Shr}, mem::transmute};

use crate::{slot::{Slot, TypedSlot}, number::NumberSlot, bigint::BigintSlot};

impl Add for Slot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            _ => unimplemented!("throw error"),
        }
    }
}

impl Sub for Slot {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            _ => unimplemented!("throw error"),
        }
    }
}

impl Mul for Slot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            _ => unimplemented!("throw error"),
        }
    }
}

impl Div for Slot {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            _ => unimplemented!("throw error"),
        }
    }
}

impl Neg for Slot {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.into_typed() {
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitAnd for Slot {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            

            _ => unimplemented!("throw error"),
        }
    }
}

impl BitOr for Slot {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            
            _ => unimplemented!("throw error"),
        }
    }
}

impl BitXor for Slot {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
           
            _ => unimplemented!("throw error"),
        }
    }
}

impl Shl for Slot {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
            

            _ => unimplemented!("throw error"),
        }
    }
}

impl Shr for Slot {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self.into_typed(), rhs.into_typed()) {
           
            _ => unimplemented!("throw error"),
        }
    }
}

impl Slot {
    pub fn strict_equal_internal(&self, other: &Self) -> bool {
        match (self.into_typed(), other.into_typed()) {


            (_, _) => false,
        }
    } 

    pub fn strict_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            Slot::new_true()
        } else {
            Slot::new_false()
        }
    }

    pub fn strict_not_equal(&self, other: &Self) -> Self {
        if self.strict_equal_internal(other) {
            Slot::new_false()
        } else {
            Slot::new_true()
        }
    }

    pub fn less_than(self, other: Self) -> Self {
        match (self.into_typed(), other.into_typed()) {
            
            
            (_, _) => panic!("TODO: type error")
        }
    }

    pub fn less_than_or_equal(self, other: Self) -> Self {
        unimplemented!("lte")
    }

    pub fn greater_than_or_equal(self, other: Self) -> Self {
        unimplemented!("gte")
    }

    pub fn greater_than(self, other: Self) -> Self {
        unimplemented!("gt")
    }    
}

impl ToString for Slot {
    fn to_string(&self) -> String {
        match self.into_typed() {
            TypedSlot::Number(num) => {
                format!("Number{}", num.to_string())
            },
            TypedSlot::Reference(reference) => {
                unimplemented!("reference to string")
            },
        }
    }
}