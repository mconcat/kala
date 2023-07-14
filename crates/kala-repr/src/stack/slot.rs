use core::{slice};
use std::{ops::{Add, Sub, Mul, Div, Deref}};

use crate::reference::{ReferenceSlot, ReferenceHeader};

use super::{pointer::{np32, wp64, null32}, number::NumberSlot, string::StringSlot, constant::ConstantSlot, bigint::BigintSlot};

#[repr(u64)]
pub enum SlotTag {
	// Values that can (possibly) be stored in heap
	Reference = 0b_0000_0000,

	Number = 0b_0000_0001,
	String = 0b_0000_0011,
	Bigint = 0b_0000_0101,
	Constant = 0b_0000_0111,
}

impl SlotTag {
	pub fn attach(self, value: u64) -> u64 {
		value & !SLOT_TAG_MASK | self as u64
	}

	pub fn detach(value: u64) -> u64 {
		value & SLOT_TAG_MASK
	}
}

const SLOT_TAG_MASK: u64 = 0b_0000_0000_0000_0000_0000_0000_0000_0111;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SlotValue(i64);

impl SlotValue {
	pub fn with_tag(tag: SlotTag, value: i32) -> Self {
		Self((value as i64) << 32 | tag as i64)
	}

	pub fn as_number_value(&self) -> i32 {
		(self.0>>32) as i32
	}

	pub fn as_bigint_value(&self) -> i32 {
		(self.0>>32) as i32
	}

	pub fn as_bigint_sign_length(&self) -> i32 {
		(self.0>>32) as i32
	}

	pub fn as_constant_value(&self) -> u32 {
		(self.0>>32) as u32
	}

	pub fn as_string_length(&self) -> u32 {
		(self.0>>32) as u32
	}

	pub fn as_reference_metadata(&self) -> u32 {
		(self.0>>32) as u32
	}
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SlotPointer(pub u64);

impl SlotPointer {
	pub fn narrow_pointer<T>(tag: SlotTag, ptr: *const T) -> Self {
		Self(ptr as u64 | tag as u64)
	}

	pub fn composed_pointer<T>(tag: SlotTag, ptr: *const T, value: i32) -> Self {
		Self(((value as i64) << 32) as u64 | ptr as u64 | tag as u64)
	}

	pub fn tag(&self) -> SlotTag {
		unsafe {std::mem::transmute(self.0 & SLOT_TAG_MASK)}
	}

	pub fn untag(&self) -> Self {
		Self(self.0 & !SLOT_TAG_MASK)
	}

	pub unsafe fn as_number_pointer(&self) -> *mut i128 {
		std::mem::transmute(self.0 & !(SlotTag::Number as u64))
	}

	pub unsafe fn as_bigint_pointer(&self, sign_len: i32) -> (i32, *const [u64]) {
		let sign = sign_len.signum();
		let len = sign_len.abs() as usize;
		(sign, std::mem::transmute(std::slice::from_raw_parts((self.0 & !(SlotTag::Number as u64)) as *const u64, len)))
	}

	pub unsafe fn as_string_pointer(&self, len: u32) -> *const str {
		std::mem::transmute(std::slice::from_raw_parts((self.0 & !(SlotTag::Number as u64)) as *const u64, len as usize))
	}

	pub unsafe fn as_reference_pointer(&self) -> *const ReferenceHeader {
		std::mem::transmute(self.0 & !(SlotTag::Reference as u64))
	}
}



#[repr(u64)]
pub enum TypedSlot {
	Reference(ReferenceSlot) = SlotTag::Reference as u64,
	Number(NumberSlot) = SlotTag::Number as u64,
	Bigint(BigintSlot) = SlotTag::Bigint as u64,
	String(StringSlot) = SlotTag::String as u64,
	Constant(ConstantSlot) = SlotTag::Constant as u64,
}



#[repr(C)]
pub union Slot {
	pub value: SlotValue,
	pub pointer: SlotPointer,
}

impl Slot {
	pub fn new_uninitalized() -> Self {
		Self {
			pointer: SlotPointer(SlotTag::Uninitialized as u64),
		}
	}

	pub fn new_null() -> Self {
		unsafe{std::mem::zeroed()}
	}

	pub fn new_undefined() -> Self {
		Self {
			pointer: SlotPointer(SlotTag::Undefined as u64),
		}
	}

	pub fn new_false() -> Self {
		Self {
			value: SlotValue::with_tag(SlotTag::Constant, 0),
		}
	}

	pub fn new_true() -> Self {
		Self {
			value: SlotValue::with_tag(SlotTag::Constant, 1),
		}
	}

	pub fn new_number_inline(value: i32) -> Self {
		Self {
			value: SlotValue::with_tag(SlotTag::Number, value),
		}
	}

	pub fn new_bigint_inline(value: i32) -> Self {
		Self {
			value: SlotValue::with_tag(SlotTag::Bigint, value),
		}
	}

	pub fn new_number(ptr: *const i128) -> Self {
		Self {
			pointer: SlotPointer::narrow_pointer(SlotTag::Number, ptr),
		}
	}

	pub fn new_bigint(ptr: *const u64, sign_len: i32) -> Self {
		Self {
			pointer: SlotPointer::composed_pointer(SlotTag::Bigint, ptr, sign_len),
		}
	}

	pub fn new_string(ptr: *const str, len: u32) -> Self {
		Self {
			pointer: SlotPointer::narrow_pointer(SlotTag::String, ptr),
		}
	}

	pub fn new_reference(ptr: *const ReferenceHeader) -> Self {
		Self {
			pointer: SlotPointer::narrow_pointer(SlotTag::Reference, ptr),
		}
	}

	pub fn into_typed(&self) -> TypedSlot {
		let tag = self.pointer.tag();
		let untag = self.pointer.untag();
		unsafe{std::mem::transmute((tag, untag))}
	}

	// Assumes that the slot is not uninitialized
	pub fn is_falsy(&self) -> bool {
		self.pointer.untag().0 == 0
	}

	// Assumes that the slot is not uninitialized
	pub fn is_truthy(&self) -> bool {
		self.pointer.untag().0 != 0
	}
}