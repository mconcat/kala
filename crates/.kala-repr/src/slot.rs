use std::{mem::{transmute, uninitialized, ManuallyDrop}, fmt::{LowerHex, Debug}, sync::Once};

use utils::SharedString;

use std::marker::ConstParamTy;
use std::sync::LazyLock;

use crate::{number::{NumberSlot, NumberObject}, object::ObjectSlot, reference::{ReferenceSlot, Reference}, constant::{slot::ConstantSlot, primitive::PrimitiveConstant}};

// unwrapping any Slot with this value will throw an error
// must be manually checked in all cases of unwrapping
pub const UNINITIALIZED: Slot = Slot { raw: 0 };

// has type tag of Reference, so typeof NULL == typeof Reference
// pub static NULL_LOCK: LazyLock<Slot> = LazyLock::new(|| Slot::new_reference(Object::new(Vec::new())));

// pub const EMPTY_STRING: LazyLock<Slot> = LazyLock::new(|| Slot::new_string(&[]));

// pub const ZERO_NUMBER: Slot = Slot(0x0000_0000_0000_0001);

// pub const ZERO_BIGINT: Slot = Slot(0x0000_0000_0000_0003);


// We want to put let/const information on the slot itself in order to express mutability/immutability not only for the variables but also for the object properties
// Javascript right now does not have a way to express this, but there is a proposal afaik to make immutable objects(tuples/records), and it would be beneficial to make this happen with help of type checker
// TODO: maybe name them Mutable*/Immutable* instead?
#[repr(usize)]
#[derive(PartialEq, Clone, Copy, ConstParamTy, Eq)]
pub enum SlotTag {
	// Heap allocated values. 
	Reference = 0b_0000,

	// Inlined small numbers. 30bits/62bits
	Number = 0b_0001,

	// Constant values. Undefined, Null, Boolean.
	Constant = 0b_0011,
/* 
	InlineReference = 0b_0100, // boolean; still heap allocated
	InlineNumber = 0b_0101, // number within 28bit/60bit 
	// InlineString = 0b_1010, // reserved
	InlineBigint = 0b_0111, // bigint within 28bit/60bit
	*/
}
/* 
#[repr(usize)]
#[derive(PartialEq)]
pub enum PrimitiveType {
	Object,
	Number,
	String,
	Bigint,

	Boolean,
	Undefined,
}
*/
// only used for transmuting into TypedSlot
#[repr(C)]
struct TaggedTypedSlot {
	tag: SlotTag,
	slot: Slot,
}

impl SlotTag {
	pub fn attach(self, value: usize) -> usize {
		value & !SLOT_TAG_MASK | self as usize
	}

	pub fn detach(value: usize) -> usize {
		value & SLOT_TAG_MASK
	}
}

#[repr(usize)]
pub enum TypedSlot {
	Object(ObjectSlot) = 0b0000,
	Reference(ReferenceSlot) = 0b0010,
	Number(NumberSlot) = 0b0001,
	Constant(ConstantSlot) = 0b0011,
}

const SLOT_TAG_MASK: usize = 0b_0000_0000_0000_0000_0000_0000_0000_1111;

#[repr(C)]
pub union Slot {
	pub raw: usize,
	pub number: ManuallyDrop<NumberSlot>,
	pub object: ManuallyDrop<ObjectSlot>,
	pub reference: ManuallyDrop<ReferenceSlot>,
	pub constant: ManuallyDrop<ConstantSlot>, 
} 

impl Drop for Slot {
	fn drop(&mut self) {
		match self.into_typed() {
			TypedSlot::Number(num) => {
				drop(num)
			},
			TypedSlot::Constant(constant) => {
				drop(constant)
			},
			TypedSlot::Reference(reference) => {
				drop(reference)
			},
			TypedSlot::Object(object) => {
				drop(object)
			},
		}
	}
}

impl Clone for Slot {
	fn clone(&self) -> Self {
		Slot{raw: unsafe{self.raw}}
	}
}

impl Debug for Slot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unimplemented!("asdf")
	}
}

impl Slot {
	// ConstantSlot and NumberSlots are inlined shortcuts,
	// which are not heap allocated
	// ObjectSlot and ReferenceSlot are heap allocated
	// To capture a variable inside a closure, inlined variable must be promoted into a heap allocated value
	// NumberSlot gets promoted into Reference::Number
	// ConstantSlot gets promoted into either
	// - undefined -> Reference::Undefined
	// - null -> Reference::Null
	// - false, true -> Reference::Boolean
	// - comptime string -> Reference::String
	pub fn capture(&mut self) -> Self {
		match self.into_typed() {
			TypedSlot::Number(num) => {
				ReferenceSlot::new_number(num).into()
			},
			TypedSlot::Constant(constant) => {
				match *constant.0 {
					PrimitiveConstant::Undefined => ReferenceSlot::new_undefined().into(),
					PrimitiveConstant::Null => ReferenceSlot::new_null().into(),
					PrimitiveConstant::False => ReferenceSlot::new_false().into(),
					PrimitiveConstant::True => ReferenceSlot::new_true().into(),
				}
			},
			TypedSlot::Object(object) => Slot { object: ManuallyDrop::new(object), }
			TypedSlot::Reference(reference) => Slot { reference: ManuallyDrop::new(reference) }
		}
	}

	pub fn raw_equal(&self, other: &Self) -> bool {
		unsafe{self.raw == other.raw}
	}

	pub fn tag(&self) -> SlotTag {
		unsafe{transmute(self.pointer.0 as usize & SLOT_TAG_MASK)}
	}

/* 	
	fn new_inline(value: i32, tag: SlotTag) -> Self {
		// TODO: tuple might not be repr(C)?
		// make a raw struct for this
		let res = unsafe{transmute(RawSlot{value, pointer: Ref::null(tag)})}; 
		println!("new_inline: {:#x}", res);
		Slot(res)
	}
*/
	pub fn new_boolean(value: bool) -> Self {
		if value {
			Self::new_true()
		} else {
			Self::new_false()
		}
	}

	pub fn new_uninitalized() -> Self {
		UNINITIALIZED
	}
	pub fn new_null() -> Self {
		ConstantSlot::new_null().into()
	}

	pub fn new_undefined() -> Self {
		ConstantSlot::new_undefined().into()
	}

	pub fn new_false() -> Self {
		ConstantSlot::new_false().into()
	}

	pub fn new_true() -> Self {
		ConstantSlot::new_true().into()
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
	pub fn into_typed(&self) -> TypedSlot {
		unsafe{transmute(TaggedTypedSlot {
			tag: self.tag(),
			slot: self.clone(),
		})}
	}

	fn is_uninitialized(&self) -> bool {
		self.raw_equal(&UNINITIALIZED)
	}

	// returns false slot if undefined, null, false, 0, 0n, empty string
	pub fn is_falsy(&self) -> bool {
	
	}

	pub fn is_truthy(&self) -> bool {
		!self.is_falsy()
	}

	pub fn is_nullish(self) -> bool {
		unimplemented!("asdf")
	}

	pub fn new_array(elements: Vec<Slot>) -> Self {
		ReferenceSlot::new_array(elements).into()
	}

	pub fn new_integer(i: i64) -> Self {
		if i < ((isize::MAX>>4) as i64) && i > ((isize::MIN>>4) as i64) {
			NumberSlot::new(i as isize).into()
		} else {
			NumberObject::new(i as i128).into()
		}
	}

	pub fn new_number_from_parts(i: i64, f: u64) -> Self {
		NumberObject::new((i as i128) << 64 + (f as i128)).into()
	}

	pub fn new_string(s: SharedString) -> Self {
		ReferenceSlot::new_string(s).into()
	}

	pub fn new_object(names: Vec<SharedString>, inlines: Vec<Slot>) -> Self {
		ObjectSlot::new(names, inlines).into()
	}

	pub fn new_function<Code>(name: Option<SharedString>, code: Code, captures: Vec<Slot>) -> Self {
		ReferenceSlot::new_function(name, code, captures).into()
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

	pub fn as_number(&self) -> Option<NumberSlot> {
		match self.into_typed() {
			TypedSlot::Number(num) => Some(num),
			_ => None,
		}
	}

	pub fn as_smi(&self) -> Option<isize> {
		match self.into_typed() {
			TypedSlot::Number(num) => Some(num),
			TypedSlot::Reference(reference) => {
				match *reference.0.as_ref() {
					unimplemented!("asdf")
				}
			},
		}	
	}

	pub fn get_element(&mut self, index: u32) -> Option<&mut Slot> {
		match self.into_typed() {
			TypedSlot::Reference(reference) => {
				match *reference {
					Reference::Array(ref mut array) => {
						array.get_element(index)
					},
					_ => None,
				}
			},
		}

	}

	pub fn get_property(&mut self, name: SharedString) -> Option<&mut Slot> {
		match self.into_typed() {
			TypedSlot::Object(object) => {
				object.get_property(name)
			},
		}
	}
}

impl PartialEq for Slot {
	fn eq(&self, other: &Self) -> bool {
		self.strict_equal_internal(other)
	}
}

impl LowerHex for Slot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:#x}", unsafe{self.pointer.0} as usize)
	}
}