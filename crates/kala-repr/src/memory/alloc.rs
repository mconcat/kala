use std::{alloc::{self, AllocError, Layout}, ptr::{NonNull, from_raw_parts}, cell::Cell, mem::MaybeUninit, sync::Once, marker::PhantomData, ops::Deref};

const CHUNK_SIZE_BITS: usize = 65536; // in bits, 65536 / 8 = 8192 bytes

const CHUNK_SIZE_BYTES: usize = CHUNK_SIZE_BITS / 8;

const CHUNK_SIZE_LOG: usize = 13; // 2 << 13 = 65536 / 8

// Raw pointer to a memory location
pub struct Ref<T: ?Sized> {
    ptr: u32,
    phantom: PhantomData<T>,
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mem = memory();

    }
}

pub struct Allocator();

unsafe impl alloc::Allocator for Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        memory().allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        memory().deallocate(ptr, layout)   
    }
}

fn memory() -> &'static Memory {
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
        SINGLETON.assume_init_ref()
    }
}

pub struct Memory {
    chunks: Cell<Vec<Chunk>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            chunks: Cell::new(vec![Chunk::new()]),
        }
    }
}

impl Memory {
    fn allocate(&self, layout: Layout) -> Result<u32, AllocError> {
        if layout.size() > CHUNK_SIZE_BYTES {
            return Err(AllocError);
        }

        let chunk = self.chunks.get_mut().last_mut().unwrap();

        if let Ok(ptr) = chunk.allocate(layout) {
            return Ok(ptr);
        }

        let mut new_chunk = Chunk::new();
        let res = new_chunk.allocate(layout);
        self.chunks.get_mut().push(new_chunk);
        res
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // do nothing
        return
    }

    fn deref<T>(&self, ptr: u32) -> Option<T> {
        let chunk_index = ptr as usize >> CHUNK_SIZE_LOG;
        let chunk = self.chunks.get_mut().get(chunk_index)?;
        let memory_index = ptr as usize % CHUNK_SIZE_BYTES
    }
}

struct Chunk {
    len: usize,
    data: [u8; CHUNK_SIZE_BYTES],
}

impl Chunk {
    fn new() -> Self {
        Self {
            len: 0,
            data: unsafe{std::mem::MaybeUninit::uninit().assume_init()},
        }
    }

    fn allocate(&mut self, layout: Layout) -> Result<u32, AllocError> {
        if self.len + layout.size() > CHUNK_SIZE_BYTES {
            return Err(AllocError);
        }

        let ptr = unsafe{self.data.as_ptr().add(self.len)};

        self.len += layout.size();

        Ok(ptr)
    }

    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        return 
    }
}



