use std::{marker::PhantomData, ops::Deref};

use crate::reference::ReferenceHeader;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct null32 {
    inner: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct full32 {
    inner: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct np32<T: Sized>{
    inner: u32,
    phantom: PhantomData<*mut T>,
}

impl<T> Deref for np32<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.is_null() {
            panic!("Attempted to dereference a null pointer");
        }
        unsafe { std::mem::transmute(self.inner as u64) }
    }
}

impl<T> np32<T> {
    pub fn new(value: T) -> Self {
        let ptr = Box::leak(Box::new(value)) as *mut T;
        let p64 = ptr as u64;
        let p32 = p64.try_into().unwrap();

        Self {
            inner: p32,
            phantom: PhantomData,
        }
    }

    pub fn null() -> Self {
        Self {
            inner: 0,
            phantom: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.inner == 0
    }

    pub fn as_raw_pointer(&self) -> *mut T {
        if self.is_null() {
            panic!("Attempted to dereference a null pointer");
        }
        unsafe { std::mem::transmute(self.inner as u64) }
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct wp64<T: Sized>{
    inner: u64,
    phantom: PhantomData<*const [T]>,
}

const LOW_32_MASK: u64 = 0x0000_0000_FFFF_FFFF;

impl<T> Deref for wp64<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let addr: *const T = unsafe {std::mem::transmute(self.inner & LOW_32_MASK)};
        let len = ((self.inner >> 32) as i32).abs() as usize;
        unsafe { std::mem::transmute(std::slice::from_raw_parts(addr, len)) }
    }
}

impl<T: Clone> wp64<T> {
    pub fn new(value: &[T]) -> Self {
        let vec_ref = Vec::leak(Vec::from(value));
        let p32: u32 = (vec_ref.as_ptr() as u64).try_into().unwrap();
        let len: u32 = (vec_ref.len() as u64).try_into().unwrap();

        Self {
            inner: ((len as u64) << 32) | (p32 as u64),
            phantom: PhantomData,
        }
    }
}