use std::{alloc::{Allocator, Layout, AllocError}, ptr::NonNull};

use super::memory::Memory;

impl Allocator for Memory {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let res = self.arena.allocate(layout)?;
        match u32::try_from(res.as_ptr() as usize) {
            Ok(ptr) => Ok(NonNull::new(ptr).unwrap()),
            Err(_) => Err(AllocError),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.arena.deallocate(ptr, layout)
    }
}

