use std::{cell::Ref, mem::{size_of, size_of_val}};

use utils::{FxMap, VectorMap, SharedString, Map};

use crate::{slot::{Slot, UNDEFINED}};

// https://doc.rust-lang.org/beta/nomicon/exotic-sizes.html
pub struct Object {
    // prototype: Slot,
    properties: VectorMap<Slot>,
    elements: Vec<Slot>,
    inlines: Vec<Slot>,
}

impl Object {
    pub fn new(inlines: Vec<Slot>) -> Self {
        Self {
            properties: VectorMap::new(),
            elements: Vec::new(),
            inlines,
        }
    }

    pub fn inline_property(&mut self, index: usize) -> Option<&mut Slot> {
        self.inlines.get_mut(index)
    }

    pub fn property(&mut self, name: SharedString) -> Option<&mut Slot> {
        self.properties.get(&name)
    }

    pub fn get_index(&self, index: i32) -> Slot {
        if index < 0 {
            panic!("negative index")
        }

        if index < self.elements.len() as i32 {
            return self.elements[index as usize]
        }

        UNDEFINED // TODO: prorotype chain
    }
}