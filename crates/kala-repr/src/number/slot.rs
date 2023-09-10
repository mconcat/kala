
use crate::{slot::{Slot, SlotTag}, memory::alloc::Ref};

use core::panic;
use std::{sync::LazyLock, ops::{Deref}, mem::transmute};

pub const POSITIVE_INFINITY_VALUE: i128 = 0x7FFF_FFFF_FFFF_FFFF;

//static POSITIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(POSITIVE_INFINITY_VALUE));

pub const NEGATIVE_INFINITY_VALUE: i128 = 0x8000_0000_0000_0000;

//static NEGATIVE_INFINITY: LazyLock<NumberSlot> = LazyLock::new(|| NumberSlot::new(NEGATIVE_INFINITY_VALUE));

// pub const NAN_SLOT: u64 = SlotTag::Number.attach(0xFFFF_FFFF_FFFF_FFFF);


#[derive(Clone)]
#[repr(C)]
pub struct NumberSlot(pub(crate) Ref<i128>);

impl NumberSlot {
	pub fn new(value: i128) -> Self {
		let pointer = Ref::new(value, SlotTag::Number);

		Self(pointer)
	}

	pub fn new_positive_infinity() -> Self {
		unimplemented!("asdf")
		//*POSITIVE_INFINITY	
	}

	pub fn new_negative_infinity() -> Self {
		unimplemented!("asdf")
		//*NEGATIVE_INFINITY
	}

	pub fn unwrap(self) -> i128 {
		*self.0
	}
}

impl Deref for NumberSlot {
	type Target = i128;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl Into<Slot> for NumberSlot {
	fn into(self) -> Slot {
		unsafe{transmute(self)}
	}
}

impl ToString for NumberSlot {
	fn to_string(&self) -> String {
		let value = *self.0;

		if value == POSITIVE_INFINITY_VALUE {
			return "Infinity".to_string()
		}

		if value == NEGATIVE_INFINITY_VALUE {
			return "-Infinity".to_string()
		}

		// check if low 64 bits are 0
		// if so, integer
		if value << 64 == 0 {
			return (value >> 64 as i64).to_string()
		}

		unimplemented!("asdf")
	}
}