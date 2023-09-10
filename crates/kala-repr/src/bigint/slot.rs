use std::{ops::Deref, ptr::slice_from_raw_parts, mem::transmute};

use crate::{slot::{Slot, SlotTag}, memory::alloc::{Ref, Slice}};

const BIGINT_INLINE_MAX: isize = isize::MAX >> 4;
const BIGINT_INLINE_MIN: isize = isize::MIN >> 4;

#[repr(C)]
pub struct BigintSlot(pub Slice<usize>);

impl BigintSlot {
	pub fn new(sign: bool, abs: Vec<usize>) -> Self {
		let sign_len: isize = if sign {abs.len() as isize} else {-(abs.len() as isize)};

		let mut pointer: Slice<usize> = Slice::new(abs.len()+1);
	}

	/* 
	pub fn new_inline(value: isize) -> Self {
		Self()
	}
	*/
/* 
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			sign_len: 0,
			pointer: Ref::with_capacity(capacity)
		}
	}
*/
	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl Into<Slot> for BigintSlot {
	fn into(self) -> Slot {
		unsafe{transmute(self)}
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