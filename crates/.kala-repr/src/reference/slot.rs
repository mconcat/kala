use std::{mem::transmute, rc::Rc, cell::Cell, ops::{DerefMut, Deref}};

use utils::SharedString;

use crate::{slot::{SlotTag, Slot}, memory::r#ref::{Ref, MASK}};

use super::Reference;

pub struct ReferenceSlot(pub(crate) Rc<Cell<Reference>>);

impl Deref for ReferenceSlot {
    type Target = Reference;

    fn deref(&self) -> &Self::Target {
        unsafe{&*((self.0.as_ref() as *const _ as usize & MASK) as *const _)}
    }
}

impl DerefMut for ReferenceSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{&mut *((self.0.as_ref() as *const _ as usize & MASK) as *mut _)}
    }
}

impl ReferenceSlot {
    pub fn new(reference: Reference) -> Self {
        Self(Rc::new(Cell::new(reference)))
    }

    pub fn new_function<Code>(name: Option<SharedString>, code: Code, captures: Vec<Slot>) -> Self {
        Self::new(Reference::Function(name, code, captures))
    }

    pub fn new_undefined() -> Self {
        Self::new(Reference::Undefined)
    }

    pub fn new_null() -> Self {
        Self::new(Reference::Null)
    }

    pub fn new_false() -> Self {
        Self::new(Reference::Boolean(false))
    }

    pub fn new_true() -> Self {
        Self::new(Reference::Boolean(true))
    }

    pub fn new_string(s: SharedString) -> Self {
        Self::new(Reference::String(s))
    }

    pub fn new_number(num: f64) -> Self {
        Self::new(Reference::Number(num))
    }
}

impl Into<Slot> for ReferenceSlot {
    fn into(self) -> Slot {
        unsafe{transmute(self)}
    }
}