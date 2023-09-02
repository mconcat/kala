
use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};

use std::{sync::LazyLock, ops::{Deref}, mem::transmute};

pub const POSITIVE_INFINITY_VALUE: i128 = 0x7FFF_FFFF_FFFF_FFFF;

//static POSITIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(POSITIVE_INFINITY_VALUE));

pub const NEGATIVE_INFINITY_VALUE: i128 = 0x8000_0000_0000_0000;

//static NEGATIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(NEGATIVE_INFINITY_VALUE));

// pub const NAN_SLOT: u64 = SlotTag::Number.attach(0xFFFF_FFFF_FFFF_FFFF);

#[derive(Clone)]
#[repr(C)]
pub struct NumberSlot {
	pub value: isize,
	pub pointer: Ref<i128>, // *mut i128 
}

impl NumberSlot {
	pub fn new(value: i128) -> Self {
		let pointer = Ref::new(value, SlotTag::Number);

		Self{ value: 0, pointer }
	}

	pub fn new_inline(value: isize) -> Self {
		Self{ value, pointer: Ref::null(SlotTag::Number) }
	}

	pub fn new_positive_infinity() -> Self {
		unimplemented!("asdf")
		//*POSITIVE_INFINITY	
	}

	pub fn new_negative_infinity() -> Self {
		unimplemented!("asdf")
		//*NEGATIVE_INFINITY
	}

	pub fn is_inline(&self) -> bool {
		self.pointer.is_null() 
	}
}

impl Into<Slot> for NumberSlot {
	fn into(self) -> Slot {
		unsafe{transmute(self)}	
	}	
}

impl Into<i128> for NumberSlot {
	fn into(self) -> i128{
		if self.pointer.is_null() {
			println!("null pointer");
			self.value.try_into().unwrap()
		} else {
			println!("non-null pointer");
			*self.pointer	
		}
	}
}
