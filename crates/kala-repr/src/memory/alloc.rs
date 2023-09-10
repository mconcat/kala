use std::{alloc::{self, Layout, Allocator, GlobalAlloc, System}, cell::Cell, mem::{MaybeUninit, size_of, transmute}, sync::Once, marker::PhantomData, ops::{Deref, DerefMut, Index, IndexMut}, slice::{from_raw_parts, from_raw_parts_mut}, ptr::NonNull, rc::Rc};

use utils::SharedString;

use crate::slot::{SlotTag, Slot};
// This allocator wraps the default global allocator (Rust's allocator)
// and adds support for tagged pointers
// - checking layout is at least 8 bytes aligned
struct TaggedPointerAllocator;

unsafe impl GlobalAlloc for TaggedPointerAllocator {
    // we assume allocation will be aligned in 16 bits,
    // due to 128bit number being the most smallest heap allocated object

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() < 16 {
            return System.alloc(Layout::from_size_align_unchecked(layout.size(), 16))
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc((ptr as usize & !0b1111) as *mut u8, layout)
    }
}

#[global_allocator]
static GLOBAL: TaggedPointerAllocator = TaggedPointerAllocator;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Slice<T>{
    words: Ref<u8>, // [len: size_of<usize>(), ...]
    phantom: PhantomData<[T]>,
}

impl Slice<u8> {
    pub fn from_shared_string(str: SharedString) -> Self { 
        let slice: Self = Slice::new(str.len());
        let data_start = unsafe{slice.words.0.add(size_of::<usize>())};

        unsafe{data_start.copy_from(str.as_bytes().as_ptr(), str.len())};

        slice
    }
}

impl<T> Slice<T> {


    pub fn new(len: usize) -> Self {
        // TODO: bound check len is 31/63bits

        let vec_len = size_of::<usize>()+size_of::<T>()*len;

        let mut vec: Vec<u8> = Vec::with_capacity(vec_len);

        unsafe{vec.set_len(vec_len)};

        let vec_ptr = vec.leak() as *mut [u8];

        let mut size_ptr = vec_ptr as *mut usize;

        unsafe{*size_ptr = len};

        Self {
            words: Ref(vec_ptr as *mut u8),
            phantom: PhantomData,
        }
    }
/* 
    pub fn new_inline(value: usize) -> Self {
        // TODO: bound check value is 31/63bits
        let mut vec: Vec<isize> = vec![-(value as isize)];


    }
    */

    pub fn len(&self) -> usize {
        unsafe{*transmute::<Ref<u8>, Ref<usize>>(self.words)}
    }
}

impl<T> Index<usize> for Slice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe{&*(self.words.as_slice(self.len()).as_ptr().add(size_of::<usize>()+index*size_of::<T>()) as *const T)}
    }
}

impl<T> IndexMut<usize> for Slice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe{&mut*(self.words.as_slice(self.len()).as_mut_ptr().add(size_of::<usize>()+index*size_of::<T>()) as *mut T)}
    }
}

// Raw pointer to a memory location
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
        let ptr = unsafe{&mut*(((self.0 as usize) & !0b1111) as *mut T)};

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

        unsafe{&*(((self.0 as usize) & !0b1111) as *const T)}
    }
}

impl<T> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_null() {
            panic!("null pointer dereference");
        }

        unsafe{&mut*(((self.0 as usize) & !01111) as *mut T)}        
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