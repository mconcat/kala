use std::{ops::{Index, IndexMut}, fmt::Debug, rc::Rc};

use crate::{function::Frame, completion::Completion};

use super::slot::Slot;

#[derive(Clone)]
pub struct Property {
    pub key: Rc<str>,
    pub data: Slot,
    pub getter: Slot,
    pub setter: Slot,
}

impl Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.getter != Slot::UNINITIALIZED {
            write!(f, "[get {:?}]", self.getter)?
        }
        if self.setter != Slot::UNINITIALIZED {
            write!(f, "[set {:?}]", self.setter)?
        }
        if self.data != Slot::UNINITIALIZED {
            write!(f, "[{} {:?}]", self.key.to_string(), self.data)?
        }
        Ok(())
    }
}

impl Property {
    pub fn data(key: impl Into<Rc<str>>, data: Slot) -> Property {
        Property {
            key: key.into(),
            data,
            getter: Slot::UNINITIALIZED,
            setter: Slot::UNINITIALIZED,
        }
    }

    pub fn accessor(key: impl Into<Rc<str>>, getter: Slot, setter: Slot) -> Property {
        Property {
            key: key.into(),
            data: Slot::UNINITIALIZED,
            getter,
            setter,
        }
    }

    pub fn getter(key: impl Into<Rc<str>>, getter: Slot) -> Property {
        Property {
            key: key.into(),
            data: Slot::UNINITIALIZED,
            getter,
            setter: Slot::UNINITIALIZED,
        }
    }

    pub fn setter(key: impl Into<Rc<str>>, setter: Slot) -> Property {
        Property {
            key: key.into(),
            data: Slot::UNINITIALIZED,
            getter: Slot::UNINITIALIZED,
            setter,
        }
    }

    pub fn get(&mut self, frame: &mut Frame) -> Completion {
        if self.getter != Slot::UNINITIALIZED {
            self.getter.call(frame, &mut vec![])
        } else {
            Completion::Value(self.data.clone())
        }
    }

    pub fn set(&mut self, frame: &mut Frame, value: Slot) -> Completion {
        if self.setter != Slot::UNINITIALIZED {
            self.setter.call(frame, &mut vec![value])
        } else {
            self.data = value;
            Completion::Value(self.data.clone())
        }
    }
}

#[derive(Clone)]
pub struct Object {
    pub properties: Vec<Property>,
}

impl Object {
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

impl Object {
    pub fn index_property_by_string(&self, index: impl Into<Rc<str>>) -> Option<&Property> {
        let index = index.into();
        for property in &self.properties {
            if property.key == index {
                return Some(property);
            }
        }

        None
    }

    pub fn index_mut_property_by_string(&mut self, index: impl Into<Rc<str>>) -> Option<&mut Property> {
        let index = index.into();
        for property in &mut self.properties {
            if property.key == index {
                return Some(property);
            }
        }

        None
    }
}