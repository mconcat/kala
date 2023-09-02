use std::{alloc::{self, Layout, Allocator, GlobalAlloc, System}, cell::Cell, mem::{MaybeUninit, size_of, transmute}, sync::Once, marker::PhantomData, ops::{Deref, DerefMut, Index, IndexMut}, slice::{from_raw_parts, from_raw_parts_mut}, ptr::NonNull, rc::Rc};

use crate::slot::{SlotTag, Slot};
// This allocator wraps the default global allocator (Rust's allocator)
// and adds support for tagged pointers
// - checking layout is at least 8 bytes aligned
struct TaggedPointerAllocator;

unsafe impl GlobalAlloc for TaggedPointerAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() < 8 {
            return System.alloc(Layout::from_size_align_unchecked(layout.size(), 8))
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc((ptr as usize & !0b0111) as *mut u8, layout)
    }
}

#[global_allocator]
static GLOBAL: TaggedPointerAllocator = TaggedPointerAllocator;

// Raw pointer to a memory location
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ref<T>(pub *mut T);
// needs to become
// pub struct Ref<'a, T>(pub &'a Cell<T>);

impl<T: Copy> Ref<T> {
    pub fn new_vec(len: usize, tag: SlotTag) -> Self {
        let vec = vec![unsafe{MaybeUninit::<T>::uninit().assume_init()}; len];

        let mut ptr = Vec::leak(vec) as *mut [T] as *mut T as usize;

        ptr |= tag as usize;

        Self(ptr as *mut T)        
    }
}

impl<T> Ref<T> {
    pub const fn null(tag: SlotTag) -> Self {
        Self(0 as *mut T)
    }

    pub fn is_null(&self) -> bool {
        self.0 as usize == 0
    }

    pub fn new(value: T, tag: SlotTag) -> Self {
        /* 
        let mem = memory();
        let layout = unsafe{Layout::from_size_align_unchecked(size_of::<T>(), 8)};
        let ptr = mem.allocate(layout).unwrap();
        println!("layout: {:?}, ptr: {ptr}", layout);
        let mut r = Self { ptr, phantom: PhantomData };
        *r = value; // TODO: optimize
        r
        */
        let mut ptr = Box::leak(Box::new(value)) as *mut T as usize;
        ptr |= tag as usize;
        Self(ptr as *mut T)
    }
    /* 
    pub fn new_length(value: Vec<T>) -> Self {
        let mem = memory();
        let layout = unsafe{
            Layout::from_size_align_unchecked(size_of::<T>()*value.len(), 8)
        };
        let ptr = mem.allocate(layout).unwrap();
        let mut r = Self { ptr, phantom: PhantomData };
        for (i, v) in value.into_iter().enumerate() {
            
        }
    }
    */

    pub fn as_slice(&mut self, len: usize) -> &mut [T] {
        let ptr = unsafe{&mut*(((self.0 as usize) & !0b0111) as *mut T)};

        unsafe{from_raw_parts_mut(ptr, len)}
    }
/* 
    pub fn new_tagged(value: T, tag: SlotTag) -> Self {
        let mut res  = Self::new(value);
        println!("ptr: {}, tag: {}", res.ptr, tag.clone() as u32);
        res.ptr |= tag as u32; // assumes 8bytes alignment, tag space initialized to be 0
        println!("tagged ptr: {}", res.ptr);
        res
    }
*/    
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.is_null() {
            panic!("null pointer dereference");
        }

        unsafe{&*(((self.0 as usize) & !0b0111) as *const T)}
    }
}

impl<T> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_null() {
            panic!("null pointer dereference");
        }

        unsafe{&mut*(((self.0 as usize) & !0b0111) as *mut T)}        
    }
}
/*
impl<T: Sized> Index<u32> for Ref<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let offset = Self{
            ptr: self.ptr+index,
            phantom: PhantomData,
        };
        &*offset 
    }
}

impl<T: Sized> IndexMut<u32> for Ref<T> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        let mut offset = Self{
            ptr: self.ptr+index,
            phantom: PhantomData,
        };
        &mut*offset 
    }
}
*/