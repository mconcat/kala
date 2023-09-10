use std::mem::{transmute, ManuallyDrop};

use utils::{VectorMap, Map, SharedString};

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref, array};

use super::{Object, Shape, Prototype};

#[repr(C)]
pub struct ReferenceSlot(pub(crate) Ref<Object>);

impl Clone for ReferenceSlot {
    fn clone(&self) -> Self {
        Self(Ref(self.0.0))
    }
}

impl ReferenceSlot {
    pub fn is_empty(&self) -> bool {
        unimplemented!("is_empty")
    }

    pub fn new_function<Code>(name: Option<SharedString>, code: Code, captures: Vec<Slot>) -> Self {
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

        Self(pointer)
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

        Self(pointer)
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

        Self(pointer)
    }

    pub fn get_property(&mut self, name: SharedString) -> Option<&mut Slot> {
        self.0.properties.get(name.clone())
    }

    pub fn get_inline_property(&self, index: i32) -> Slot {
        unimplemented!("inline property")
    }

    pub fn get_element<'a>(&'a mut self, index: i32) -> Option<&'a mut Slot> {
        let array_element = self.0.get_index(index);
        
        if array_element.is_some() {
            return array_element
        }
        unimplemented!("stringified access for numeric property")
/* 
        let object_property = self.get_property(SharedString::from_string(index.to_string()));

        object_property
        */
    }

}

impl Into<Slot> for ReferenceSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }   
}

impl PartialEq for ReferenceSlot {
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}