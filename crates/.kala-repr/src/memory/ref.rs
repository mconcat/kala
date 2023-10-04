use std::ops::{Deref, DerefMut};

use crate::{slot::SlotTag};

pub const ALIGN: usize = 8;
pub const MASK: usize = !0b0111;

#[derive(Clone)]
pub struct Ref<T, const Tag: SlotTag>(pub(crate) Box<T>);

impl<T, const Tag: SlotTag> Ref<T, Tag> {
    pub fn new(value: T) -> Self {
        let mut ptr = Box::into_raw(Box::new(value));
        ptr = ((ptr as usize) | (Tag as usize)) as *mut T;
        let boxed = unsafe{Box::from_raw(ptr)};

        Self(boxed)
    }
}
/* 
impl<T> Deref for Ref<T, {SlotTag::Object}> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()        
    }
}

impl<T> DerefMut for Ref<T, {SlotTag::Object}> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}
*/

impl<T, const Tag: SlotTag> Deref for Ref<T, Tag> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{&*((self.0.as_ref() as *const T as usize & MASK) as *const T)}
    }
}

impl<T, const Tag: SlotTag> DerefMut for Ref<T, Tag> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{&mut*((self.0.as_mut() as *mut T as usize & MASK) as *mut T)}
    }
}

impl<T, const Tag: SlotTag> Drop for Ref<T, Tag> {
    fn drop(&mut self) {
        drop(unsafe{Box::from_raw((Box::into_raw(self.0) as *mut T as usize & MASK) as *mut T)});
    }
}