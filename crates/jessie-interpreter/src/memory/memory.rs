use bumpalo::Bump;

// 64-bit aligned memory with 32-bit pointers
// we assume that the vm will run on 64-bit machines, but webassembly only supports 32-bit pointers
pub struct Memory {
    pub arena: Bump
}

#[derive(Clone, Copy)]
pub struct Pointer<T: ?Sized> {
    pub ptr: *mut T
}

impl<T: ?Sized> Pointer<T> {
    pub fn new(ptr: &mut T) -> Self {
        Self {
            ptr
        }
    }

    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut()
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            arena: Bump::new(),
        }
    }

    pub fn allocate<T>(&self, value: T) -> Pointer<T> {
        let ptr = self.arena.alloc(value);
        Pointer {
            ptr: ptr as *mut T,
        }
    }


    pub fn allocate_bytes<T: ?Sized>(&self, value: &[u8]) -> Pointer<T> {
        let ptr = self.arena.alloc_slice_copy(value);
        Pointer {
            ptr: ptr as *mut [u8],
        }
    }
}