use std::{ops::Deref, slice::from_raw_parts};

use crate::slot::{Slot, SlotTag};

#[repr(C)]
pub struct StringSlot {
	len: i32,
	pointer: u32,
}

impl Into<Slot> for StringSlot {
	fn into(self) -> Slot {
		Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::String as u64)
	}	
}

impl Deref for StringSlot {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		unsafe{std::str::from_utf8_unchecked(from_raw_parts(self.pointer as *const u8, self.len as usize))}
	}
}
