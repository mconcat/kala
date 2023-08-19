
use crate::slot::{Slot, SlotTag};

use std::sync::LazyLock;

pub const POSITIVE_INFINITY_VALUE: i128 = 0x7FFF_FFFF_FFFF_FFFF;

static POSITIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(POSITIVE_INFINITY_VALUE));

pub const NEGATIVE_INFINITY_VALUE: i128 = 0x8000_0000_0000_0000;

static NEGATIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(NEGATIVE_INFINITY_VALUE));

// pub const NAN_SLOT: u64 = SlotTag::Number.attach(0xFFFF_FFFF_FFFF_FFFF);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NumberSlot {
	pub value: i32,
	pub pointer: u32, // *mut i128 
}

impl NumberSlot {
	pub fn new(alloc: value: i128) -> Self {
		let ptr: usize = Box::leak(Box::new(value)) as *mut i128 as usize;

		println!("ptr: {:X}", ptr);

		let ptr_value: u32 = ptr.try_into().unwrap();

		Self{ value: 0, pointer: ptr_value & 0xFFFF_FFFD }
	}

	pub fn new_inline(value: i32) -> Self {
		Self{ value, pointer: 0 }
	}

	pub fn new_positive_infinity() -> Self {
		*POSITIVE_INFINITY	
	}

	pub fn new_negative_infinity() -> Self {
		*NEGATIVE_INFINITY
	}

	pub fn is_inline(&self) -> bool {
		self.pointer == 0
	}
}

impl Into<Slot> for NumberSlot {
	fn into(self) -> Slot {
		Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Number as u64)
	}	
}

impl Into<i128> for NumberSlot {
	fn into(self) -> i128{
		if self.pointer == 0 {
			i128::from(self.value)
		} else {
			unsafe {*(self.pointer as *mut i128)}
		}
	}
}
