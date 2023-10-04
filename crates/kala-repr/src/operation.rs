use std::{mem::ManuallyDrop, str::FromStr};

use crate::{slot::{Slot, MASK, SlotTag::{self}, SlotTag::Integer, SlotTag::Constant, SlotConstant, SlotInteger, SlotReference}, reference::Reference};

impl Slot {
    fn op_strict_equal_internal(&self, other: &Self) -> bool {
        // Fast path
        if unsafe{self.raw == other.raw} {
            return true
        }

        // Slow path
        match (self.get_tag(), other.get_tag()) {
            // forward to internal operations if same tag
            (SlotTag::Reference, SlotTag::Reference) => {
                self.unwrap_reference().op_strict_equal_internal(other.unwrap_reference())
            }
            (SlotTag::Integer, SlotTag::Integer) => {
                self.unwrap_integer().op_strict_equal_internal(&other.unwrap_integer())
            }
            (SlotTag::Constant, SlotTag::Constant) => {
                self.unwrap_constant().op_strict_equal_internal(&other.unwrap_constant())
            }

            // 
            (SlotTag::Reference, SlotTag::Integer) => {
                match self.unwrap_reference() {
                    Reference::Number(number) => number.op_strict_equal_internal_integer(&other.unwrap_integer()),
                    _ => false,
                }
            }
            (SlotTag::Integer, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Number(number) => number.op_strict_equal_internal_integer(&self.unwrap_integer()), 
                    _ => false,
                }
            }
            (SlotTag::Reference, SlotTag::Constant) => {
                match self.unwrap_reference() {
                    Reference::Constant(constant) => constant.op_strict_equal_internal(&other.unwrap_constant()),
                    _ => false,
                }
            }
            (SlotTag::Constant, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Constant(constant) => self.unwrap_constant().op_strict_equal_internal(constant),
                    _ => false,
                }
            }

            // false otherwise
            (SlotTag::Integer, SlotTag::Constant) => {
                false
            }
            (SlotTag::Constant, SlotTag::Integer) => {
                false
            }
            _ => unreachable!("invalid slot tag"),
        }
    }

    pub fn op_strict_equal(&self, other: &Self) -> Slot {
        if self.op_strict_equal_internal(other) {
            Slot::TRUE
        } else {
            Slot::FALSE
        }
    }

    pub fn op_strict_not_equal(&self, other: &Self) -> Slot {
        if self.op_strict_equal_internal(other) {
            Slot::FALSE
        } else {
            Slot::TRUE
        }
    }

    pub fn op_less_than_internal(&self, other: &Self) -> Option<bool> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Constant, SlotTag::Constant) => {
                self.unwrap_constant().op_less_than_internal(&other.unwrap_constant())
            }
            (SlotTag::Integer, SlotTag::Integer) => {
                Some(self.unwrap_integer().op_less_than_internal(&other.unwrap_integer()))
            }
            (SlotTag::Reference, SlotTag::Reference) => {
                match (self.unwrap_reference(), other.unwrap_reference()) {
                    (Reference::Number(left), Reference::Number(right)) => {
                        left.op_less_than_internal(right)
                    }
                    (Reference::Constant(left), Reference::Constant(right)) => {
                        left.op_less_than_internal(right)
                    }
                    (Reference::String(left), Reference::String(right)) => {
                        Some(*left < *right)
                    }
                    _ => None,
                }
            }

            (SlotTag::Reference, SlotTag::Integer) => {
                match self.unwrap_reference() {
                    Reference::Number(number) => number.op_less_than_internal_integer(&other.unwrap_integer()),
                    _ => None
                }
            }  
            (SlotTag::Integer, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Number(number) => unimplemented!("wrapped number object"),
                    _ => None
                }
            }
            (SlotTag::Reference, SlotTag::Constant) => {
                match self.unwrap_reference() {
                    Reference::Constant(constant) => constant.op_less_than_internal(&other.unwrap_constant()),
                    _ => None
                }
            }
            (SlotTag::Constant, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Constant(constant) => self.unwrap_constant().op_less_than_internal(constant),
                    _ => None 
                }
            }

            (SlotTag::Integer, SlotTag::Constant) => {
                None 
            }
            (SlotTag::Constant, SlotTag::Integer) => {
                None
            }
        }
    }
    pub fn op_less_than_or_equal_internal(&self, other: &Self) -> Option<bool> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Constant, SlotTag::Constant) => {
                unimplemented!("comparison of constants")
            }
            (SlotTag::Integer, SlotTag::Integer) => {
                if self.unwrap_integer().op_strict_equal_internal(&other.unwrap_integer()) {
                    return Some(true)
                }

                Some(self.unwrap_integer().op_less_than_internal(&other.unwrap_integer()))
            }
            (SlotTag::Reference, SlotTag::Reference) => {
                match (self.unwrap_reference(), other.unwrap_reference()) {
                    (Reference::Number(left), Reference::Number(right)) => {
                        if left.op_strict_equal_internal(right) {
                            return Some(true)
                        }

                        left.op_less_than_internal(right)
                    }
                    (Reference::Constant(left), Reference::Constant(right)) => {
                        if left.op_strict_equal_internal(right) {
                            return Some(true)
                        }

                        left.op_less_than_internal(right)
                    }
                    (Reference::String(left), Reference::String(right)) => {
                        Some(*left <= *right)
                    }
                
                    _ => None,
                }
            }

            (SlotTag::Reference, SlotTag::Integer) => {
                match self.unwrap_reference() {
                    Reference::Number(number) => number.op_less_than_internal_integer(&other.unwrap_integer()),
                    _ => None
                }
            }
            (SlotTag::Integer, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Number(number) => unimplemented!("wrapped number object"),
                    _ => None
                }
            }
            (SlotTag::Reference, SlotTag::Constant) => {
                match self.unwrap_reference() {
                    Reference::Constant(constant) => constant.op_less_than_internal(&other.unwrap_constant()),
                    _ => None
                }
            }
            (SlotTag::Constant, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Constant(constant) => self.unwrap_constant().op_less_than_internal(constant),
                    _ => None 
                }
            }
            
            (SlotTag::Integer, SlotTag::Constant) => {
                None 
            }
            (SlotTag::Constant, SlotTag::Integer) => {
                None
            }
        }
    }

    pub fn op_less_than(&self, other: &Self) -> Slot {
        match self.op_less_than_internal(other) {
            Some(true) => Slot::TRUE,
            Some(false) => Slot::FALSE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_less_than_or_equal(&self, other: &Self) -> Slot {
        match self.op_less_than_or_equal_internal(&other) {
            Some(true) => Slot::TRUE,
            Some(false) => Slot::FALSE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_greater_than(&self, other: &Self) -> Slot {
        match other.op_less_than_or_equal_internal(&self) {
            Some(true) => Slot::FALSE,
            Some(false) => Slot::TRUE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_greater_than_or_equal(&self, other: &Self) -> Slot {
        match other.op_less_than_internal(&self) {
            Some(true) => Slot::FALSE,
            Some(false) => Slot::TRUE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_add_internal(&self, other: &Self) -> Option<Slot> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Constant, SlotTag::Constant) => {
                self.unwrap_constant().op_add_internal(&other.unwrap_constant()).map(Into::into)
            }
            (SlotTag::Integer, SlotTag::Integer) => {
                self.unwrap_integer().op_add_internal(&other.unwrap_integer()).map(Into::into)
            }
            (SlotTag::Reference, SlotTag::Reference) => {
                match (self.unwrap_reference(), other.unwrap_reference()) {
                    (Reference::Number(left), Reference::Number(right)) => {
                        let result = left.op_add_internal(right)?.into();
                        Some(Slot{ reference: ManuallyDrop::new(SlotReference::new(result)) })
                    }
                    (Reference::Constant(left), Reference::Constant(right)) => {
                        left.op_add_internal(right).map(Into::into)
                    }
                    (Reference::String(left), Reference::String(right)) => {
                        unimplemented!("string concatenation")
                        /* 
                        let mut string = String::from_str(left).unwrap();
                        string.push_str(&right);
                        string.into()
                        */
                    }
                    _ => None,
                }
            }

            (SlotTag::Reference, SlotTag::Integer) => {
                match self.unwrap_reference() {
                    Reference::Number(number) => {
                        let result = number.op_add_internal_integer(&other.unwrap_integer())?.into();
                        Some(Slot{ reference: ManuallyDrop::new(SlotReference::new(result)) })
                    }
                    _ => None
                }
            }
            (SlotTag::Integer, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Number(number) => unimplemented!("wrapped number object"),
                    _ => None
                }
            }
            (SlotTag::Reference, SlotTag::Constant) => {
                match self.unwrap_reference() {
                    Reference::Constant(constant) => constant.op_add_internal(&other.unwrap_constant()).map(Into::into),
                    _ => None
                }
            }
            (SlotTag::Constant, SlotTag::Reference) => {
                match other.unwrap_reference() {
                    Reference::Constant(constant) => self.unwrap_constant().op_add_internal(constant).map(Into::into),
                    _ => None 
                }
            }

            (SlotTag::Integer, SlotTag::Constant) => {
                None 
            }
            (SlotTag::Constant, SlotTag::Integer) => {
                None
            }
        }
    }
}