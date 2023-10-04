
use crate::{slot::{Slot, SlotTag}, memory::r#ref::Ref, reference::{ReferenceSlot, Reference}};

use core::panic;
use std::{sync::LazyLock, ops::{Deref}, mem::transmute};

pub const POSITIVE_INFINITY_VALUE: i128 = 0x7FFF_FFFF_FFFF_FFFF;

//static POSITIVE_INFINITY: LazyLock<NumberObject> = LazyLock::new(|| NumberObject::new(POSITIVE_INFINITY_VALUE));

pub const NEGATIVE_INFINITY_VALUE: i128 = 0x8000_0000_0000_0000;

//static NEGATIVE_INFINITY: LazyLock<NumberObject> = LazyLock::new(|| NumberObject::new(NEGATIVE_INFINITY_VALUE));

// pub const NAN_SLOT: u64 = SlotTag::Number.attach(0xFFFF_FFFF_FFFF_FFFF);

// Fixed128
// conceptually equivalent to i128, but split into four parts due to memory align problem and for long arithmetics
// u32s are 2s complement if negative
// orders are reversed to perserve endianess
#[derive(Clone)]
#[repr(C)]
pub struct NumberObject(pub(crate) Box<F128>);

#[derive(Clone)]
#[repr(C)]
pub struct F128(pub(crate) i128);

impl NumberObject {
	pub fn new(value: F128) -> Self {
		Self(Box::new(value))
	}

	pub fn new_positive_infinity() -> Self {
		unimplemented!("asdf")
		//*POSITIVE_INFINITY	
	}

	pub fn new_negative_infinity() -> Self {
		unimplemented!("asdf")
		//*NEGATIVE_INFINITY
	}
}

impl Into<Slot> for NumberObject {
	fn into(self) -> Slot {
		ReferenceSlot::new(Reference::Number(self)).into()
	}
}

impl ToString for NumberObject {
	fn to_string(&self) -> String {
		let value = self.0;

		if value.0 == i128::MAX {
			return "Infinity".to_string()
		}

		if value.0 == i128::MIN {
			return "-Infinity".to_string()
		}

		// check if low 64 bits are 0
		// if so, integer
		if value.0 & 0xFFFF_FFFF_FFFF_FFFF == 0 {
			return (value.0 >> 64 as i64).to_string()
		}

		unimplemented!("asdf")
	}
}