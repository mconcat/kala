use utils::{FxMap, VectorMap, SharedString, Map};

use crate::slot::Slot;

#[derive(Clone)]
pub struct Shape {
    pub inlines: VectorMap<usize>,
}

// https://doc.rust-lang.org/beta/nomicon/exotic-sizes.html

#[derive(Clone)]
pub struct Object {
    // pub(crate) prototype: Prototype,
    // pub(crate) shape: Shape,
    pub(crate) properties: VectorMap<Slot>, 
    // pub(crate) inlines: Box<[Slot]>, // TODO: Slice<Slot>
}

impl Object {
    pub fn new() -> Self {
        Self {
            //prototype: Prototype::object(),
            shape: Shape {
                inlines: VectorMap::new(),
            },
            properties: VectorMap::new(),
            inlines: Box::new([]),
        }
    }

    /* 
    pub fn inline_property(&mut self, index: usize) -> Option<&mut Slot> {
        self.inlines.get_mut(index)
    }
    */

    pub fn property(&mut self, name: SharedString) -> Option<&mut Slot> {
        let inline_index = self.shape.inlines.get(name.clone()).copied();
        if inline_index.is_some() {
            return self.inlines.get_mut(inline_index.unwrap())
        }

        self.properties.get(name.clone())
    }
/* 
    pub fn get_index(&mut self, index: u32) -> Option<&mut Slot> {
        if let Prototype::Array(array) = &mut self.prototype {
            return array.elements.get_mut(index as usize)
        }  

        None
        // TODO: prorotype chain
    }
    */
}

