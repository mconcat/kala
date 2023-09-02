use std::{ops::Deref, str::from_utf8, mem::transmute, cell::Cell};

use utils::SharedString;

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};

#[repr(C)]
pub struct StringSlot {
	len: isize,
	pointer: Cell<Ref<u8>>,
}

impl StringSlot {
	pub fn new(s: SharedString) -> Self {
		let mut pointer: Ref<u8> = Ref::new_vec(s.len(), SlotTag::String);

		let len = s.len();

		pointer.as_slice(len).copy_from_slice(s.as_bytes());
		
		Self {
			len: len as isize,
			pointer: Cell::new(pointer),
		}
	}
}

impl Into<Slot> for StringSlot {
	fn into(self) -> Slot {
		unsafe{transmute(self)}
	}	
}

impl Deref for StringSlot {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		let slice = unsafe{&mut*self.pointer.as_ptr()}.as_slice(self.len as usize);

		let s = from_utf8(slice).unwrap();
		s
	}
}

impl PartialEq for StringSlot {
	fn eq(&self, other: &Self) -> bool {
		self.deref() == other.deref()
	}
}
