use core::{slice};
use std::{ops::{Add, Sub, Mul, Div, Deref}, ptr::slice_from_raw_parts, mem::transmute};

use utils::SharedString;

use crate::{reference::{ReferenceSlot, Object}, number::NumberSlot, bigint::BigintSlot, string::slot::StringSlot};

use std::sync::LazyLock;

// unwrapping any Slot with this value will throw an error
// must be manually checked in all cases of unwrapping
pub const UNINITIALIZED: Slot = Slot(0xFFFF_FFFF_0000_0000); 

// has type tag of Reference, so typeof NULL == typeof Reference
// pub static NULL_LOCK: LazyLock<Slot> = LazyLock::new(|| Slot::new_reference(Object::new(Vec::new())));

pub const UNDEFINED: Slot = Slot(0x0000_0000_0000_0000);

pub const FALSE: Slot = Slot(0x0000_0000_0000_0002);

pub const TRUE: Slot = Slot(0x0000_0001_0000_0002);

// pub const EMPTY_STRING: LazyLock<Slot> = LazyLock::new(|| Slot::new_string(&[]));

// pub const ZERO_NUMBER: Slot = Slot(0x0000_0000_0000_0001);

// pub const ZERO_BIGINT: Slot = Slot(0x0000_0000_0000_0003);


#[repr(u64)]
#[derive(PartialEq)]
pub enum SlotTag {
	Reference = 0b_0000_0000,
	Number = 0b_0000_0001,
	String = 0b_0000_0010,
	Bigint = 0b_0000_0011,
}

#[repr(u64)]
#[derive(PartialEq)]
pub enum SlotType {
	Object = 0b_0000_0000,
	Number = 0b_0000_0001,
	String = 0b_0000_0010,
	Bigint = 0b_0000_0011,

	Boolean,
	Undefined,
}

impl Into<Slot> for SlotType {
	// for typeof()
	fn into(self) -> Slot {
		StringSlot::new(SharedString::from_str(match self {
			SlotType::Object => "object",  // TODO: null
			SlotType::Number => "number",
			SlotType::String => "string",
			SlotType::Bigint => "bigint",
			SlotType::Boolean => "boolean",
			SlotType::Undefined => "undefined"
		})).into()
	}
}

#[repr(u64)]
pub enum TypedSlot {
	Reference(ReferenceSlot) = 0b_0000_0000,
	Number(NumberSlot) = 0b_0000_0001,
	String(StringSlot) = 0b_0000_0010,
	Bigint(BigintSlot) = 0b_0000_0011,

	Uninitialized,
}

impl SlotTag {
	pub fn attach(self, value: u64) -> u64 {
		value & !SLOT_TAG_MASK | self as u64
	}

	pub fn detach(value: u64) -> u64 {
		value & SLOT_TAG_MASK
	}
}

const SLOT_TAG_MASK: u64 = 0b_0000_0000_0000_0000_0000_0000_0000_0011;

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Slot(pub u64);

impl Slot {
	pub fn tag(self) -> SlotTag {
		unsafe{std::mem::transmute(self.0 & SLOT_TAG_MASK)}
	}

	fn value(self) -> u64 {
		self.0 & 0xFFFF_FFFF_0000_0000
	}

	fn pointer(self) -> u64 {
		self.0 & 0x0000_0000_FFFF_FFFC
	}

	fn untag(self) -> Slot {
		Slot(self.0 & !SLOT_TAG_MASK)
	}
	
	fn new(value: i32, pointer: u32, tag: SlotTag) -> Self {
		Self((value as u64) << 32 | (pointer as u64) | tag as u64)
	}

	fn new_inline(value: i32, tag: SlotTag) -> Self {
		Self((value as u64) << 32 | tag as u64)
	}

	pub fn new_boolean(value: bool) -> Self {
		if value {
			TRUE
		} else {
			FALSE
		}
	}

	pub fn new_uninitalized() -> Self {
		UNINITIALIZED
	}
	pub fn new_null() -> Self {
//		*NULL_LOCK
unimplemented!("asdf")
	}

	pub fn new_undefined() -> Self {
		UNDEFINED
	}

	pub fn new_false() -> Self {
		FALSE
	}

	pub fn new_true() -> Self {
		TRUE
	}
/* 
	pub fn new_number_inline(value: i32) -> Self {
		Self::new_inline(value, SlotTag::Number)
	}

	pub fn new_bigint_inline(value: i32) -> Self {
		Self::new_inline(value, SlotTag::Bigint)
	}
	
	pub fn new_number(value: i128) -> Self {
		Self::new(0, leak_pointer(value), SlotTag::Number)
	}

	pub fn new_bigint(sign_len: i32, value: Vec<u64>) -> Self {
		Self::new(sign_len, leak_pointer(value), SlotTag::Bigint)
	}

	pub fn new_string(value: &[u8]) -> Self {
		Self::new(value.len().try_into().unwrap(), leak_pointer(value), SlotTag::String)
	}
	*/
/* 
	pub fn new_reference(value: &Object<[Slot]>) -> Self {
		Self::new((value.len().try_into::<i16>().unwrap() as i32) << 16, (Box::leak(Box::new(value)) as *const Object).try_into::<u32>().unwrap(), SlotTag::Reference)
	}
	*/
/* 
	pub fn into_number(self) -> Option<NumberSlot> {
		if self.tag() != SlotTag::Number {
			return None
		}

		let untag = self.untag();
		let value = unsafe{std::mem::transmute::<u64, *mut i128>(untag.0 as u64)};
		Some(NumberSlot::new(unsafe{*value}))
	}

	pub fn into_bigint(self) -> Option<BigintSlot> {
		if self.tag() != SlotTag::Bigint {
			return None
		}

		if self == UNINITIALIZED {
			return None
		}

		let untag = self.untag();
		let value = slice_from_raw_parts(transmute(self.), len)
	}
*/
	pub fn into_typed(self) -> TypedSlot {
		if self == UNINITIALIZED {
			return TypedSlot::Uninitialized
		}

		let tag = self.tag();
		let untag = self.untag();
		unsafe{transmute((tag, untag))}
	}

	// returns false slot if undefined, null, false, 0, 0n, empty string
	pub fn is_falsy(self) -> bool {
		match self.tag() {
			SlotTag::Reference => {
				if self == UNINITIALIZED {
					panic!("uninitialized slot")
				}

				if /*self == *NULL_LOCK ||*/ self == UNDEFINED {
					false
				} else {
					true
				}
			}
			SlotTag::Number => {
				if self.untag() == Slot(0) {
					false
				} else {
					true
				}
			},
			SlotTag::String => {
				if self.value() == 0 {
					false
				} else {
					true
				}
			},
			SlotTag::Bigint => {
				if self.untag() == Slot(0) {
					false
				} else {
					true
				}
			},
		}
	}

	pub fn is_truthy(self) -> bool {
		!self.is_falsy()
	}

	pub fn type_of(self) -> SlotType {
		if self == UNINITIALIZED {
			panic!("uninitialized slot")
		}

		let ty: SlotType = unsafe{transmute(self.tag())};

		if ty == SlotType::String {
			if self.pointer() == 0 {
				return SlotType::Boolean
			} else {
				return SlotType::String
			}
		}

		if ty == SlotType::Object {
			if self == UNDEFINED {
				return SlotType::Undefined
			} else {
				return SlotType::Object
			}
		}

		ty
	}

	pub fn is_nullish(self) -> bool {
		return self == UNDEFINED// || self == NULL
	}

	pub fn new_array(elements: Vec<Slot>) -> Self {
		ReferenceSlot::new_array(elements).into()
	}

	pub fn new_number_from_parts(i: i64, f: u64) -> Self {
		NumberSlot::new((i as i128) << 64 + (f as i128)).into()
	}

	pub fn new_string(s: SharedString) -> Self {
		StringSlot::new(s).into()
	}
/* 
	pub fn get_index(self, index: i32) -> Slot {
		if self.tag() != SlotTag::Reference {
			panic!("cannot index non-reference")
		}

		if self == UNINITIALIZED {
			panic!("uninitialized slot")
		}

		let obj = unsafe{&*(self.pointer() as *const Object)};
		obj.get_index(index)
	}
	*/

	pub fn as_reference(&self) -> Option<ReferenceSlot> {
		match self.into_typed() {
			TypedSlot::Reference(obj) => Some(obj),
			_ => None,
		}
	}

	pub fn get_element(&self, index: i32) -> Option<Slot> {
		let obj = self.as_reference()?;
		obj.get_element(index)
	}

	pub fn get_property(&self, name: SharedString) -> Option<Slot> {
		let mut obj = self.as_reference()?;
		obj.get_property(name)
	}
}

