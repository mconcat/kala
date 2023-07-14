use core::slice;
use std::{marker::PhantomData, ops::Deref};

use crate::{pointer::np32, Slot, SlotPointer, SlotTag};

pub const NULL: u64 = 0x0000_0000_0000_0000;

#[repr(C)]
pub struct ReferenceHeader {
	reference_count: u32,
	len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ref32<Body> {
    inner: u32,
    phantom: PhantomData<*mut (ReferenceHeader, [Body])>,
}

impl<Body> ref32<Body> {
    fn deref(&self) -> &(ReferenceHeader, &[Body]) {
        if self.is_null() {
            panic!("Attempted to dereference a null pointer");
        }
        let header: ReferenceHeader = unsafe { *std::mem::transmute::<u64, *const ReferenceHeader>(self.inner as u64) };
        let body: &[Body] = unsafe { std::slice::from_raw_parts(std::mem::transmute::<u64, *const Body>((self.inner + std::mem::size_of::<ReferenceHeader>() as u32) as u64), header.len()) };
        &(header, body)
    }
}

impl<Body> ref32<Body> {
    pub fn new(header: ReferenceHeader, body: &[Body]) -> Self {
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
pub struct ReferenceSlot {
	value: u32,
	pointer: np32<Slot>,
}

impl Deref for ReferenceSlot {
	type Target = (ReferenceHeader, *const [Slot]);

	fn deref(&self) -> &Self::Target {
		if self.pointer.is_null() {
			panic!("ReferenceSlot is null")
		} else {
			let header = unsafe {*(self.pointer.as_raw_pointer() as *const ReferenceHeader)};
			let len = header.len as usize;
			const header_size: usize = std::mem::size_of::<ReferenceHeader>() / std::mem::size_of::<Slot>(); // assumes both are aligned to 8 bytes
			let slice = unsafe{ slice::from_raw_parts(self.pointer.as_raw_pointer().add(header_size), len - header_size) };
			&(header, slice)
		}
	}
}

impl Into<Slot> for ReferenceSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Reference as u64) }
	}	
}