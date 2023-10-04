use std::{mem::transmute, ops::Deref};

use crate::{slot::{Slot, SlotTag}};

// Generalized in mind of future extension for bigint
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InlineNumericSlot<const Tag: SlotTag>(pub(crate) isize);// 4 bit shifted, 28/60 bit value

impl<const Tag: SlotTag> InlineNumericSlot<Tag> {
	pub fn new(value: isize) -> Self {
		let mut inline = value << 4;
		if inline >> 4 != value {
			panic!("overflow") // overflow, value is too big
		}
		inline |= Tag as isize;

		Self(inline)
	}

	pub fn unwrap(self) -> isize {
		self.0 >> 4
	}

    pub fn is_zero(&self) -> bool {
        self.unwrap() == 0
    }
}

impl<const Tag: SlotTag> Into<Slot> for InlineNumericSlot<Tag> {
	fn into(self) -> Slot {
		unsafe{transmute(self)}	
	}	
}