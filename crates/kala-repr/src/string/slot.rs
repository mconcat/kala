use std::{ops::Deref, slice::from_raw_parts, cell::Cell, str::from_utf8};

use utils::SharedString;

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};

#[repr(C)]
pub struct StringSlot {
	len: i32,
	pointer: Cell<Ref<u8>>,
}

impl StringSlot {
	pub fn new(s: SharedString) -> Self {
		let mut pointer: Cell<Ref<u8>> = Cell::new(Ref::with_capacity(s.len()));

		let len = s.len();

		(unsafe{&mut*pointer.get().as_slice(len)}).clone_from_slice(s.as_bytes());

		Self {
			len: len as i32,
			pointer,
		}
	}
}

impl Into<Slot> for StringSlot {
	fn into(self) -> Slot {
		Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::String as u64)
	}	
}

impl Deref for StringSlot {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		let bytes = unsafe{(&*self.pointer.get().as_slice(self.len as usize))};
		let s = from_utf8(bytes).unwrap();
		s
	}
}
