use super::slot::{Slot, SlotTag};

use std::ops::{Add, Sub, Mul, Div, Rem, Neg, Not, BitAnd, BitOr, BitXor, Shl, Shr, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

// https://tc39.es/ecma262/2023/#sec-evaluatestringornumericbinaryexpression

impl Slot {
    fn add_alloc(&mut self, other: Self) -> (Self, bool) {
        match (self.tag(), other.tag()) {
            (SlotTag::Number, SlotTag::Number) => {
                let a_ptr = self.get_number_pointer();
                let b_ptr = other.get_number_pointer();

                match (a_ptr.is_null(), b_ptr.is_null()) {
                    (true, true) =>  unsafe{ 
                        
                    },
                    (true, false) => {
                        unimplemented!("allocation on numbers: see unimplemented above")
                    }
                    (false, true) => {
                        unimplemented!("allocation on numbers: see unimplemented above")
                    },
                    (false, false) => {
                        unimplemented!("allocation on numbers: see unimplemented above")
                    }
                }
            }
            (SlotTag::Number, SlotTag::Bigint) => {
                
            }
        }
    }
}