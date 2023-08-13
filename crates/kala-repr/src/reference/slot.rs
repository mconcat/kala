use core::slice;
use std::{marker::PhantomData, ops::Deref, cell::Ref};

use crate::slot::{Slot, SlotTag};

use super::Object;


#[repr(C)]
pub struct ReferenceSlot {
    len: i16,
    _reserved: u16,
    pointer: u32,
}

impl ReferenceSlot {
    pub fn new(inlines: Vec<Slot>) -> Self {
        let len = inlines.len().try_into().unwrap();

        let obj = Box::leak(Box::new(Object::new(inlines)));

        let ptr: u32 = (obj as *mut Object as usize).try_into().unwrap();

        Self{ len, _reserved: 0, pointer: ptr }
    }
}

impl Into<Slot> for ReferenceSlot {
    fn into(self) -> Slot {
        Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Reference as u64)
    }   
}