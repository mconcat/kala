use std::{mem::ManuallyDrop, str::FromStr, rc::Rc};

use crate::{slot::{Slot, MASK, SlotTag::{self}, SlotConstant, SlotInteger, SlotReference}, reference::Reference, constant::Constant, integer::Integer, number::Number, object::Property};

impl Slot {
    pub fn op_add(&self, other: &Self) -> Slot {
        match self.op_add_internal(other) {
            Some(slot) => slot,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_sub(&self, other: &Self) -> Slot {
        unimplemented!("subtraction")
    }

    pub fn op_mul(&self, other: &Self) -> Slot {
        match self.op_mul_internal(other) {
            Some(slot) => slot,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_div(&self, other: &Self) -> Slot {
        unimplemented!("division")
    }

    pub fn op_modulo(&self, other: &Self) -> Slot {
        match self.op_modulo_internal(other) {
            Some(slot) => slot,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_pow(&self, other: &Self) -> Slot {
        unimplemented!("power")
    }

    pub fn op_neg(&self) -> Slot {
        match self.get_tag() {
            SlotTag::Integer => {
                let result = self.unwrap_integer().op_neg();
                Slot{ integer: ManuallyDrop::new(SlotInteger(result)) }
            },
            SlotTag::Reference => {
                match self.unwrap_reference() {
                    Reference::Number(number) => Slot{reference: ManuallyDrop::new(SlotReference::new(number.op_neg().into()))},
                    _ => unimplemented!("wrapped object"),
                }
            }
            _ => Slot::UNDEFINED, // TODO: error
        }
    }

    fn op_strict_equal_internal(&self, other: &Self) -> bool {
        // Fast path
        if unsafe{self.raw == other.raw} {
            return true
        }

        // Check if heap

        // Slow path
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Pointer, _) => {
                self.unwrap_pointer().op_strict_equal_internal(other)
            },
            (_, SlotTag::Pointer) => {
                other.unwrap_pointer().op_strict_equal_internal(self)
            },
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
            (SlotTag::Pointer, _) => {
                self.unwrap_pointer().op_less_than_internal(other)
            },
            (_, SlotTag::Pointer) => {
                other.unwrap_pointer().op_less_than_internal(self).map(|x| !x)
            },
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
            (SlotTag::Pointer, _) => {
                self.unwrap_pointer().op_less_than_internal(other)
            },
            (_, SlotTag::Pointer) => {
                other.unwrap_pointer().op_less_than_internal(self).map(|x| !x)
            },
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
            Some(true) => Slot::TRUE,
            Some(false) => Slot::FALSE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_greater_than_or_equal(&self, other: &Self) -> Slot {
        match other.op_less_than_internal(&self) {
            Some(true) => Slot::TRUE,
            Some(false) => Slot::FALSE,
            None => Slot::UNDEFINED, // TODO: error
        }
    }

    pub fn op_add_internal(&self, other: &Self) -> Option<Slot> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Pointer, _) => {
                self.unwrap_pointer().op_add_internal(other)
            },
            (_, SlotTag::Pointer) => {
                other.unwrap_pointer().op_add_internal(self)
            },
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

    fn op_mul_internal(&self, other: &Self) -> Option<Slot> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Integer, SlotTag::Integer) => {
                let (lo, hi) = self.unwrap_integer().overflowing_mul(other.unwrap_integer());

                if hi == 0 {
                    Integer::new(lo as i64).map(Into::into)
                } else {
                    unimplemented!("integer mul overflow")
                    //Some(Reference::Number(Number::new(lo as i64, hi as u64)).into())
                }
            },
            _ => None,
        }
    }

    fn op_modulo_internal(&self, other: &Self) -> Option<Slot> {
        match (self.get_tag(), other.get_tag()) {
            (SlotTag::Integer, SlotTag::Integer) => {
                Some(Slot::new_integer((self.unwrap_integer().unwrap()%other.unwrap_integer().unwrap()).try_into().unwrap()))
            },
            _ => None,
        }
    }

    pub fn op_not(&self) -> Slot {
        match self.get_tag() {
            SlotTag::Pointer => {
                self.unwrap_pointer().op_not()
            },
            SlotTag::Constant => {
                match self.unwrap_constant() {
                    Constant::Undefined => Slot::TRUE,
                    Constant::Null => Slot::TRUE,
                    Constant::False => Slot::TRUE,
                    Constant::True => Slot::FALSE,
                }
            },
            SlotTag::Integer => {
                let result = self.unwrap_integer().op_neg();
                Slot{ integer: ManuallyDrop::new(SlotInteger(result)) }
            },
            SlotTag::Reference => {
                match self.unwrap_reference() {
                    Reference::Number(number) => Slot{reference: ManuallyDrop::new(SlotReference::new(number.op_neg().into()))},
                    _ => unimplemented!("wrapped object"),
                }
            }

        }
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.op_strict_equal_internal(other)
    }
}

impl Slot {
    // [] operator
    // Retrieves the element at the given index.
    pub fn get_element(&mut self, index: usize) -> Option<&mut Slot> {
        match self.get_tag() {
            SlotTag::Reference => {
                match self.unwrap_mut_reference() {
                    Reference::Object(object) => {
                        unimplemented!("object indexing")
                    }
                    Reference::Array(array) => {
                        array.get_element(index)
                    }
                    Reference::String(string) => {
                        unimplemented!("string indexing")
                    }
                    _ => None,
                }
            }
            SlotTag::Pointer => {
                self.unwrap_mut_pointer().get_element(index)
            }
            _ => None, // TODO: wrapped objects
        }
    }

    // . operator
    // Retrieves the property with the given name.
    pub fn get_property(&mut self, name: &Rc<str>) -> Option<&mut Property> {
        match self.get_tag() {
            SlotTag::Reference => {
                match self.unwrap_mut_reference() {
                    Reference::Object(object) => {
                        object.index_mut_property_by_string(name.clone())
                    }
                    Reference::Array(array) => {
                        unimplemented!("array indexing")
                    }
                    Reference::String(string) => {
                        unimplemented!("string indexing")
                    }
                    _ => None,
                }
            },
            SlotTag::Pointer => {
                self.unwrap_mut_pointer().get_property(name)
            }
            _ => None, // TODO: wrapped objects
        }
    }
}