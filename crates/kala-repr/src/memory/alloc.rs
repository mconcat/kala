use std::{alloc::{self, Layout}, cell::Cell, mem::{MaybeUninit, size_of, transmute}, sync::Once, marker::PhantomData, ops::{Deref, DerefMut, Index, IndexMut}, slice::{from_raw_parts, from_raw_parts_mut}};

use crate::slot::SlotTag;

const CHUNK_SIZE: u32 = 4096;

const CHUNK_INDEX_MASK: u32 = CHUNK_SIZE - 1;

const CHUNK_SIZE_LOG: u32 = 12; // 2 << 12 = 4096

// u32 pointer of 0b00000000_11111111_00000000_11111111
// chunk:         0b00000000_11111111_0000
// pointer:       0b                      0000_11111111

// Raw pointer to a memory location
pub struct Ref<T: ?Sized> {
    pub(crate) ptr: u32,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Copy for Ref<T> {
}

impl<T: Sized> Ref<T> {
    pub const fn null() -> Self {
        Self { ptr: 0, phantom: PhantomData }
    }

    pub fn is_null(&self) -> bool {
        self.ptr == 0
    }

    pub fn new(value: T) -> Self {
        let mem = memory();
        let layout = unsafe{Layout::from_size_align_unchecked(size_of::<T>(), 8)};
        let ptr = mem.allocate(layout).unwrap();
        println!("layout: {:?}, ptr: {ptr}", layout);
        let mut r = Self { ptr, phantom: PhantomData };
        *r = value; // TODO: optimize
        r
    }

    pub fn offset(&self, offset: usize) -> Self {
        Self {
            ptr: self.ptr + (offset as u32),
            phantom: PhantomData,
        }
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

    pub fn as_slice(&mut self, len: usize) -> *mut [T] {
        unsafe{from_raw_parts_mut((&mut**self) as *mut T, len) as *mut [T]}
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mem = memory();
        let layout = unsafe {
            Layout::from_size_align_unchecked(size_of::<T>()*capacity, 8)
        };
        let ptr = mem.allocate(layout).unwrap();
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    pub fn new_tagged(value: T, tag: SlotTag) -> Self {
        let mut res  = Self::new(value);
        res.ptr |= (tag as u32); // assumes 8bytes alignment, tag space initialized to be 0
        res
    }
    
    fn chunk(&self) -> usize {
        (self.ptr >> CHUNK_SIZE_LOG) as usize
    }

    fn index(&self) -> usize {
        (self.ptr & CHUNK_INDEX_MASK) as usize
    }
}

impl<T: Sized> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mem = memory();
        let chunk = &mem.chunks.get_mut()[self.chunk()];
        unsafe{transmute(&chunk.data[self.index()])}
    }
}

impl<T: Sized> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mem = memory();
        let chunk = &mut mem.chunks.get_mut()[self.chunk()];
        unsafe{transmute(&mut chunk.data[self.index()])} 
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

fn memory() -> &'static mut Memory {
    // Create an uninitialized static
    static mut SINGLETON: MaybeUninit<Memory> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = Memory::new();
            // Store it to the static var, i.e. initialize it
            SINGLETON.write(singleton);
        });

        // Now we give out a shared reference to the data, which is safe to use
        // concurrently.
        SINGLETON.assume_init_mut()
    }
}

pub struct Memory {
    chunks: Cell<Vec<Box<Chunk>>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            chunks: Cell::new(vec![Box::new(Chunk::new())]),
        }
    }
}

impl Memory {
    fn allocate(&mut self, layout: Layout) -> Result<u32, ()> {
        if layout.size() > CHUNK_SIZE as usize {
            // TODO: large object alloc
            return Err(());
        }

        let chunks = self.chunks.get_mut();
        let chunk = chunks.last_mut().unwrap();

        if let Ok(index) = chunk.allocate(layout) {
            return Ok(((chunks.len() as u32)-1 << CHUNK_SIZE_LOG) + index);
        }

        let mut new_chunk = Box::new(Chunk::new());
        let index = new_chunk.allocate(layout).unwrap();
        chunks.push(new_chunk);
        return Ok(((chunks.len() as u32)-1 << CHUNK_SIZE_LOG) + index)
    }
}

struct Chunk {
    len: u32,
    data: [u8; CHUNK_SIZE as usize],
}

impl Chunk {
    fn new() -> Self {
        Self {
            len: 0,
            data: unsafe{std::mem::MaybeUninit::uninit().assume_init()},
        }
    }

    fn allocate(&mut self, layout: Layout) -> Result<u32, ()> {
        if self.len + (layout.size() as u32) > CHUNK_SIZE {
            return Err(());
        }

        let ptr = self.len;

        self.len += layout.size() as u32;

        Ok(ptr)
    }
}



