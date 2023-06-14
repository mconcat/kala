use std::alloc::{alloc, dealloc, Layout, Allocator};
use std::collections::LinkedList;
use std::marker::PhantomData;
use std::mem::size_of;

pub const BLOCK_SIZE_BITS_1K: usize = 10;
pub const BLOCK_SIZE_1K: usize = (1 << BLOCK_SIZE_BITS_1K) / size_of::<usize>();

pub const BLOCK_SIZE_BITS_32K: usize = 15;
pub const BLOCK_SIZE_32K: usize = (1 << BLOCK_SIZE_BITS_32K) / size_of::<usize>();

pub type Arena1K = Arena<BLOCK_SIZE_1K>;
pub type Arena32K = Arena<BLOCK_SIZE_32K>;


// https://rust-hosted-langs.github.io/book/chapter-simple-bump.html
// We aren't really using it for performance, as smart contract platforms bump allocate it internally anyway.
// Having a bump allocation makes the references in the AST have a fixed lifetime,
// so we can get rid of Box or Rc in the AST.

#[inline]
#[cfg(target_pointer_width="32")]
fn align_size(byte_size: usize) -> usize {
    ((byte_size - 1) >> 2) + 1
}

#[inline]
#[cfg(target_pointer_width="64")]
fn align_size(byte_size: usize) -> usize {
    ((byte_size - 1) >> 3) + 1
}

#[derive(Debug)]
pub struct Box<T: Sized> {
    ptr: *const usize,
    phantom: PhantomData<T>,
}

impl<T: Sized> Box<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*(self.ptr as *const T) }
    }

    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

#[derive(Debug)]
pub enum AllocError {
    OutOfMemory,
}

#[derive(Debug)]
pub struct Arena<const BlockSize: usize> {
    list: LinkedList<BumpBlock<BlockSize>>
}

impl<const BlockSize: usize> Arena<BlockSize> {
    pub fn new() -> Self {
        let mut list = LinkedList::new();
        list.push_front(BumpBlock::new());
        Self {
            list,
        }
    }

    pub fn allocate<T: Sized>(&mut self, data: T) -> Result<Box<T>, AllocError> {
        let alloc_size = align_size(size_of::<T>());
        let block = self.list.front_mut().unwrap();
        let alloc = block.inner_alloc(alloc_size);

        let ptr = if let Some(ptr) = alloc {
            ptr
        } else {
            let mut block = BumpBlock::new();
            let ptr = block.inner_alloc(alloc_size).unwrap();
            self.list.push_front(block);
            ptr
        };

        unsafe {
            *(ptr as *mut T) = data;
        }

        Ok(Box {
            ptr,
            phantom: PhantomData,
        })
    }
}

#[derive(Clone, Debug)]
pub struct BumpBlock<const BlockSize: usize> {
    cursor: *const usize,
    ptr: *const usize,
}

impl<const BlockSize: usize> BumpBlock<BlockSize> {
    fn new() -> Self {
        unsafe {
            let layout = Layout::new::<[usize; BlockSize]>();
            let ptr = alloc(layout) as *const usize;
            let cursor = ptr.offset(BlockSize as isize);
            Self {
                cursor,
                ptr,
            }
        }
    }

    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const usize> {
        let block_start_ptr = self.ptr as usize;
        let cursor_ptr = self.cursor as usize;

        let next_ptr = cursor_ptr.checked_sub(alloc_size)?;

        if next_ptr < block_start_ptr {
            // allocation would start lower than block beginning, which means
            // there isn't space in the block for this allocation
            None
        } else {
            self.cursor = next_ptr as *const usize;
            Some(next_ptr as *const usize)
        }
    }
}

impl<const BlockSize: usize> Drop for BumpBlock<BlockSize> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<[usize; BlockSize]>();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
