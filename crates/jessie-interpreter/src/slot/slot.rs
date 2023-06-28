use core::slice;
use std::ops::Add;

use crate::memory::memory::Pointer;

#[repr(u32)]
pub enum SlotTag {
	Reference = 0b_0000_0000,
	Undefined = 0b_0000_0010,
	_ReservedWeakReference = 0b_0000_0100,
	Uninitialized = 0b_0000_0110,
	
	Number = 0b_0000_0001,
	Constant = 0b_0000_0011,
	String = 0b_0000_0101,
	Bigint = 0b_0000_0111,
}

// UnsafeSlot is a union of all possible values that can be stored in a slot
#[repr(C)]
pub struct Slot {
	pub value: i32,
	pub pointer: u32,
}

impl Slot {
	pub unsafe fn null() -> Self {
		std::mem::zeroed()
	}

	pub fn tag(&self) -> SlotTag {
		unsafe {std::mem::transmute(self.value & 0b_0000_0111)}
	}

	pub fn get_pointer(&self) -> *mut u64 {
		unsafe{std::mem::transmute((self.pointer & !0b_0000_0111) as u64)}
	}

	pub fn get_slot_pointer(&self) -> *mut Slot {
		unsafe{std::mem::transmute((self.pointer & !0b_0000_0111) as u64) }
	}

	// not sure if [u64; 2] has the same layout with (i64, u64)
	// check it later
	pub unsafe fn get_number_pointer(&self) -> *const i128 {
		std::mem::transmute((self.pointer & !0b_0000_0111) as u64)
	}

	pub unsafe fn get_bigint_pointer(&self) -> *const [u64] {
		std::mem::transmute(slice::from_raw_parts(std::mem::transmute::<usize, *const u64>((self.pointer & !0b_0000_0111) as usize), self.value as usize))
	}

	fn is_zero(&self) -> bool {
		// i32: value is 0, pointer is empty
		// 64.64: value is 0, pointer is empty
		// bigint32: value is 0, pointer is empty
		// bigint: length is 0, pointer is null(not allocated)
		// string: length is 0, pointer is null(not allocated)
		// reference: metadata is empty, pointer is null(not allocated)
		// constant: false is 0, pointer is null(not allocated)
		// undefined: 0
		// TODO: negative zero
		self.into_word() == 0
	}

	pub fn into_word(&self) -> u64 {
		unsafe {std::mem::transmute(self)}
	}

	pub fn is_falsy(&self) -> bool {
		self.remove_tag().is_zero()
	}

	pub fn is_truthy(&self) -> bool {
		!self.is_falsy()
	}

	pub fn remove_tag(&self) -> Self {
		let mut slot = *self;
		slot.pointer = slot.pointer & !0b_0000_0111;
		slot
	}

	pub const fn new_zero() -> Self {
		Self{
			value: 0,
			pointer: SlotTag::Number as u32,
		}
	}

	pub fn new_i32(value: i32) -> Self {
		Self{
			value,
			pointer: SlotTag::Number as u32,
		}
	}

	pub fn new_number(pointer: *mut u64) -> Self {
		let pointer = u32::try_from(pointer as u64).unwrap();
		Self{
			value: 0,
			pointer: pointer | SlotTag::Number as u32,
		}
	}

	pub fn new_b32(value: i32) -> Self {
		Self{
			value,
			pointer: SlotTag::Bigint as u32,
		}
	}

	pub fn new_bigint(sign_len: i32, abs_pointer: *mut u64) -> Self {
		let pointer = u32::try_from(abs_pointer as u64).unwrap();
		Self{
			value: sign_len,
			pointer: pointer | SlotTag::Bigint as u32,
		}
	}

	pub fn new_string(pointer: *mut str) -> Self {
		let raw_parts = pointer.to_raw_parts();
		let pointer = u32::try_from(pointer as u64).unwrap();
		Self{
			value: len,
			pointer: pointer | SlotTag::String as u32,
		}
	}

	pub fn new_constant(value: ConstantValue) -> Self {
		Self{
			value: value as i32,
			pointer: SlotTag::Constant as u32,
		}
	}

	pub const fn new_uninitalized() -> Self {
		Self{
			value: ConstantValue::Uninitialized as i32,
			pointer: SlotTag::Constant as u32,
		}
	}

	pub const fn new_undefined() -> Self {
		Self{
			value: ConstantValue::Undefined as i32,
			pointer: SlotTag::Constant as u32,
		}
	}

	pub const fn new_true() -> Self {
		Self{
			value: ConstantValue::True as i32,
			pointer: SlotTag::Constant as u32,
		}
	}

	pub const fn new_false() -> Self {
		Self{
			value: ConstantValue::False as i32,
			pointer: SlotTag::Constant as u32,
		}
	}

	pub const fn new_null() -> Self {
		Self{
			value: 0,
			pointer: SlotTag::Reference as u32,
		}
	}

	pub fn new_reference(pointer: *mut u64) -> Self {
		let pointer = u32::try_from(pointer as u64).unwrap();
		Self{
			value: 0,
			pointer: pointer | SlotTag::Reference as u32,
		}
	}
}

#[repr(i32)]
pub enum ConstantValue {
	Uninitialized = -1,
	Undefined = 0,
	True = 1,
	False = 2,
}