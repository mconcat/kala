use std::mem::ManuallyDrop;

use crate::slot::{Slot, SlotConstant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constant {
    Undefined = 0b0000_0011,
    Null = 0b0000_0111,
    True = 0b0000_1011,
    False = 0b0000_1111,
}

impl Constant {
    pub fn is_nullish(&self) -> bool {
        match self {
            Constant::Undefined | Constant::Null => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Constant::Undefined | Constant::Null | Constant::False => false,
            Constant::True => true,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn op_strict_equal_internal(&self, other: &Self) -> bool {
        self == other
    }

    pub fn op_less_than_internal(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Constant::Undefined, _) => None,
            (_, Constant::Undefined) => None,
            (Constant::Null, _) => None,
            (_, Constant::Null) => None,
            (Constant::False, Constant::True) => Some(true),
            (Constant::True, Constant::False) => Some(false),
            (Constant::False, Constant::False) => Some(false),
            (Constant::True, Constant::True) => Some(false),
        }
    }

    pub fn op_add_internal(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Constant::Undefined, _) => None,
            (_, Constant::Undefined) => None,
            (Constant::Null, _) => None,
            (_, Constant::Null) => None,
            (Constant::False, Constant::True) => None,
            (Constant::True, Constant::False) => None,
            (Constant::False, Constant::False) => None,
            (Constant::True, Constant::True) => None,
        }
    }
}

impl ToString for Constant {
    fn to_string(&self) -> String {
        match self {
            Constant::Undefined => "undefined".to_string(),
            Constant::Null => "null".to_string(),
            Constant::True => "true".to_string(),
            Constant::False => "false".to_string(),
        }
    }
}

impl Into<Slot> for Constant {
    fn into(self) -> Slot {
        Slot{
            constant: ManuallyDrop::new(SlotConstant(self)),
        }
    }
}