use std::{ops::Deref, ptr::slice_from_raw_parts};

use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};


#[repr(C)]
pub struct BigintSlot {
	pub(crate) sign_len: i32,
	pub(crate) pointer: Ref<u64>, // start of the slice
}

impl BigintSlot {
	pub fn new_inline(value: i32) -> Self {
		Self {
			sign_len: value,
			pointer: Ref::null(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			sign_len: 0,
			pointer: Ref::with_capacity(capacity)
		}
	}

	pub fn is_inline(&self) -> bool {
		self.pointer.is_null()	
	}

	pub fn len(&self) -> usize {
		self.sign_len.abs() as usize
	}
}

impl Into<Slot> for BigintSlot {
	fn into(self) -> Slot {
		Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Bigint as u64)
	}
}
/* 
impl<'a> Into<(bool, &'a [u64])> for &'a BigintSlot {
	fn into(self) -> (bool, &'a [u64]) {
		if self.is_inline() {
			return (self.sign_len.is_negative(), &[self.pointer.ptr as u64])
		}

		let arr = unsafe{&*slice_from_raw_parts(&*self.pointer, self.len())};
		(self.sign_len.is_negative(), arr)
	}
}
*/