use std::mem::transmute;

use utils::{VectorMap, Map, SharedString};

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref, function::Variable};

use super::{Object, Shape, Prototype};


#[repr(C)]
pub struct ReferenceSlot {
    _reserved: isize,
    pub(crate) pointer: Ref<Object>,
}

impl Clone for ReferenceSlot {
    fn clone(&self) -> Self {
        Self {
            _reserved: 0,
            pointer: Ref(self.pointer.0), 
        }
    }
}

impl ReferenceSlot {
    pub fn new_function<Code>(name: SharedString, code: Code, captures: Vec<Variable>) -> Self {
        let mut pointer: Ref<Object> = Ref::new(
            Object {
                prototype: Prototype::function(name, code, captures),
                shape: Shape {
                    inlines: VectorMap::new(),
                },
                properties: VectorMap::new(),
                inlines: Box::new([]), 
            },
            SlotTag::Reference,
        );

        Self {
            _reserved: 0,
            pointer,
        }
    }

    pub fn new_array(elements: Vec<Slot>) -> Self {
        let mut pointer: Ref<Object> = Ref::new(
            Object {
                prototype: Prototype::array(elements),
                shape: Shape {
                    inlines: VectorMap::new(),
                },
                properties: VectorMap::new(),
                inlines: Box::new([]), 
            },
            SlotTag::Reference,
        );

        Self {
            _reserved: 0,
            pointer,
        }
    }

    pub fn new_object(names: Vec<SharedString>, inlines: Vec<Slot>) -> Self {
        let mut shape_inlines = VectorMap::new();

        for (i, name) in names.into_iter().enumerate() {
            shape_inlines.insert(name.clone(), i).unwrap();
        }

        let mut pointer: Ref<Object> = Ref::new(
            Object {
                prototype: Prototype::object(),
                shape: Shape {
                    inlines: shape_inlines, 
                },
                properties: VectorMap::new(),
                inlines: inlines.into_boxed_slice(),
            },
            SlotTag::Reference,
        );

        Self {
            _reserved: 0,
            pointer: unsafe{transmute(pointer)},
        }
    }

    pub fn get_property(&mut self, name: SharedString) -> Option<Slot> {
        self.pointer.properties.get(name.clone()).cloned()
    }

    pub fn get_inline_property(&self, index: i32) -> Slot {
        unimplemented!("inline property")
    }

    pub fn get_element(&mut self, index: i32) -> Option<Slot> {
        self.pointer.get_index(index).or_else(|| self.get_property(SharedString::from_string(index.to_string())))
    }

}

impl Into<Slot> for ReferenceSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }   
}