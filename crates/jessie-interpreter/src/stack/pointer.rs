use std::{marker::PhantomData, ops::Deref};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct null32 {
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

pub trait ReferenceHeader {
    fn len(&self) -> usize;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ref32<Header, Body> {
    inner: u32,
    phantom: PhantomData<*mut (Header, [Body])>,
}

impl<Header: ReferenceHeader, Body> ref32<Header, Body> {
    fn deref(&self) -> &(Header, &[Body]) {
        if self.is_null() {
            panic!("Attempted to dereference a null pointer");
        }
        let header: Header = unsafe { *std::mem::transmute::<u64, *const Header>(self.inner as u64) };
        let body: &[Body] = unsafe { std::slice::from_raw_parts(std::mem::transmute::<u64, *const Body>((self.inner + std::mem::size_of::<Header>() as u32) as u64), header.len()) };
        &(header, body)
    }
}

impl<Header, Body> ref32<Header, Body> {
    pub fn new(header: Header, body: &[Body]) -> Self {
        let ptr = Box::new((header, *body));
        
        let (p64, _) = ptr.to_raw_parts();
        let p32 = (p64 as u64).try_into().unwrap();

        Self {
            inner: p32,
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