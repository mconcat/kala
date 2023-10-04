use std::mem::transmute;

use crate::{memory::r#ref::Ref, slot::{SlotTag, Slot}};

use super::primitive::PrimitiveConstant;

pub struct ConstantSlot(pub(crate) Ref<PrimitiveConstant, {SlotTag::Constant}>);

impl ConstantSlot {
    pub fn new_undefined() -> Self {
        Self(Ref::new(PrimitiveConstant::Undefined))
    }

    pub fn new_null() -> Self {
        Self(Ref::new(PrimitiveConstant::Null))
    }

    pub fn new_false() -> Self {
        Self(Ref::new(PrimitiveConstant::False))
    }

    pub fn new_true() -> Self {
        Self(Ref::new(PrimitiveConstant::True))
    }
}

impl Into<Slot> for ConstantSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}