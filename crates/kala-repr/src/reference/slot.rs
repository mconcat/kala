use core::slice;
use std::{marker::PhantomData, ops::Deref, mem::{size_of, transmute}};

use utils::{VectorMap, Map, SharedString};

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};

use super::{Object};


#[repr(C)]
#[derive(Clone)]
pub struct ReferenceSlot {
    _reserved: i32,
    pointer: Ref<Object>,
}

impl ReferenceSlot {
    pub fn new_array(elements: Vec<Slot>) -> Self {
        let mut pointer: Ref<Object> = Ref::new(
            Object {
                properties: VectorMap::new(),
                elements,
                inlines: Box::new([]), 
            }
        );

        Self {
            _reserved: 0,
            pointer,
        }
    }

    pub fn new(inlines: Vec<Slot>) -> Self {
        let mut pointer: Ref<Object> = Ref::new(
            Object {
                properties: VectorMap::new(),
                elements: Vec::new(),
                inlines: inlines.into_boxed_slice(),
            }
        );

        Self {
            _reserved: 0,
            pointer,
        }
    }

    pub fn get_property(&mut self, name: SharedString) -> Option<Slot> {
        self.pointer.properties.get(&name).copied()
    }

    pub fn get_inline_property(&self, index: i32) -> Slot {
        unimplemented!("inline property")
    }

    pub fn get_element(&self, index: i32) -> Option<Slot> {
        self.pointer.elements.get(index as usize).copied()
    }

}

impl Into<Slot> for ReferenceSlot {
    fn into(self) -> Slot {
        Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Reference as u64)
    }   
}