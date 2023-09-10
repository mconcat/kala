use std::{mem::transmute, ops::Deref};

use crate::{memory::alloc::Ref, slot::{SlotTag, Slot}};

#[derive(Clone, Copy, PartialEq)]
pub enum InlineObject {
    Undefined,
    Null,
    False,
    True,
}

pub struct InlineReferenceSlot(pub(crate) Ref<InlineObject>);

impl InlineReferenceSlot {
    pub fn new(value: InlineObject) -> Self {
        Self(Ref::new(value, SlotTag::InlineReference))
    }

}

impl Clone for InlineReferenceSlot {
    fn clone(&self) -> Self {
        Self(Ref(self.0.0))
    }
}

impl Into<Slot> for InlineReferenceSlot {
    fn into(self) -> Slot {
        unsafe { transmute(self) }
    }
}

impl Deref for InlineReferenceSlot {
    type Target = InlineObject;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for InlineReferenceSlot {
    fn to_string(&self) -> String {
        match self.deref() {
            InlineObject::Undefined => "undefined".to_string(),
            InlineObject::Null => "null".to_string(),
            InlineObject::False => "false".to_string(),
            InlineObject::True => "true".to_string(),
        }
    }
}

impl PartialEq for InlineReferenceSlot {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}
