use std::{ops::Deref, ptr::slice_from_raw_parts, mem::transmute};

use crate::{slot::{Slot, SlotTag}};

const BIGINT_INLINE_MAX: isize = isize::MAX >> 4;
const BIGINT_INLINE_MIN: isize = isize::MIN >> 4;

// first word is always the sign_len
#[repr(transparent)]
pub struct BigintObject(pub(crate) Box<[usize]>);

impl BigintObject {
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

/* 
impl<'a> Into<(bool, &'a [u64])> for &'a BigintObject {
	fn into(self) -> (bool, &'a [u64]) {
		if self.is_inline() {
			return (self.sign_len.is_negative(), &[self.pointer.ptr as u64])
		}

		let arr = unsafe{&*slice_from_raw_parts(&*self.pointer, self.len())};
		(self.sign_len.is_negative(), arr)
	}
}
*/