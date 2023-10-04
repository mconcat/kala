use std::ops::{Index, IndexMut};

use utils::SharedString;

use super::slot::Slot;

pub struct Property {
    pub key: SharedString,
    pub value: Slot,
}

pub struct Object {
    pub properties: Vec<Property>,
}

impl Object {
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

impl Object {
    pub fn index_property_by_string(&self, index: SharedString) -> Option<&Slot> {
        for property in &self.properties {
            if property.key == index {
                return Some(&property.value);
            }
        }

        None
    }

    pub fn index_mut_property_by_string(&mut self, index: SharedString) -> Option<&mut Slot> {
        for property in &mut self.properties {
            if property.key == index {
                return Some(&mut property.value);
            }
        }

        None
    }
}