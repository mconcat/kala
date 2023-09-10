use std::{mem::transmute, fmt::LowerHex, sync::Once};

use utils::SharedString;

use crate::{reference::{ReferenceSlot, InlineReferenceSlot, InlineObject}, number::{NumberSlot, InlineNumberSlot}, bigint::{BigintSlot, InlineBigintSlot}, string::slot::StringSlot, memory::alloc::Ref};
use std::marker::ConstParamTy;
use std::sync::LazyLock;

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
	Reference = 0b_0000, // uninitialized(0x0), objects, 
	Number = 0b_0001, // number
	String = 0b_0010, // string
	Bigint = 0b_0011, // bigint

	InlineReference = 0b_0100, // undefined, null, boolean; still heap allocated
	InlineNumber = 0b_0101, // number within 28bit/60bit 
	// InlineString = 0b_1010, // reserved
	InlineBigint = 0b_0111, // bigint within 28bit/60bit
}

impl SlotTag {
	pub fn into_type(self) -> SlotType {
		unsafe{transmute(self as u64 & 0b_0000_0011)}
	}

	pub fn is_mutable(self) -> bool {
		(self as u64 & 0b_0000_0100) == 0
	}
}

#[repr(usize)]
#[derive(PartialEq)]
pub enum SlotType {
	Object,
	Number,
	String,
	Bigint,

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

#[repr(usize)]
pub enum TypedSlot {
	Reference(ReferenceSlot) = 0b_0000,
	Number(NumberSlot) = 0b_0001,
	String(StringSlot) = 0b_0010,
	Bigint(BigintSlot) = 0b_0011,

	InlineReference(InlineReferenceSlot) = 0b_0100,
	InlineNumber(InlineNumberSlot) = 0b_0101,
	//InlineString(InlineStringSlot) = 0b_0110,
	InlineBigint(InlineBigintSlot) = 0b_0111,
}

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

const SLOT_TAG_MASK: usize = 0b_0000_0000_0000_0000_0000_0000_0000_1111;

#[repr(C)]
#[derive(Clone, Copy)]
pub union Slot {
	pub raw: usize,
	pub pointer: Ref<()>,
} 

impl Slot {
	// capture basically does:
	// 1. Heap allocate all the inline slots
	// 2. Modify the existing slot to point the heap
	// 3. Make a new slot that points the same heap address
	// 4. Make 
	// This works because we don't have deallocation right now, once we have it the capturing should involve reference counting(or make the heap allocated object reference count by default)
	pub fn capture(&mut self) -> Self {
		match self.into_typed() {
			TypedSlot::Number(num) => {
				self.clone()
			},
			TypedSlot::Bigint(num) => {
				self.clone()
			},
			TypedSlot::String(string) => {
				self.clone()
			},
			TypedSlot::Reference(reference) => {
				self.clone()
			},
			TypedSlot::InlineReference(reference) => {
				self.clone()
			}
			// TODO: inlines should be promoted to non-inlines and then captured
			_ => panic!("cannot capture inline slot")
		}
	}

	pub fn raw_equal(&self, other: &Self) -> bool {
		self.raw == other.raw	
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
//		*NULL_LOCK
unimplemented!("asdf")
	}

	pub fn new_undefined() -> Self {
		InlineReferenceSlot::new(InlineObject::Undefined).into()
	}

	pub fn new_false() -> Self {
		InlineReferenceSlot::new(InlineObject::False).into()
	}

	pub fn new_true() -> Self {
		InlineReferenceSlot::new(InlineObject::True).into()
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
		match self.into_typed() {
			TypedSlot::Number(num) => false,
			TypedSlot::InlineNumber(num) => num.0.is_zero(),
			TypedSlot::String(string) => string.is_empty(),
			TypedSlot::Bigint(bigint) => false,
			TypedSlot::InlineBigint(bigint) => bigint.0.is_zero(),
			TypedSlot::Reference (reference) => reference.is_empty(),
			TypedSlot::InlineReference (reference) => match *reference {
				InlineObject::Undefined => true,
				InlineObject::Null => true,
				InlineObject::False => true,
				InlineObject::True => false,
			} 
		}
	}

	pub fn is_truthy(&self) -> bool {
		!self.is_falsy()
	}

	pub fn type_of(&self) -> SlotType {
		match self.into_typed() {
			TypedSlot::Bigint(_) => SlotType::Bigint,
			TypedSlot::InlineBigint(_) => SlotType::Bigint,
			TypedSlot::Number(_) => SlotType::Number,
			TypedSlot::InlineNumber(_) => SlotType::Number,
			TypedSlot::String(_) => SlotType::String,
			TypedSlot::Reference(_) => SlotType::Object,
			TypedSlot::InlineReference(reference) => match *reference {
				InlineObject::Undefined => SlotType::Undefined,
				InlineObject::Null => SlotType::Object,
				InlineObject::False => SlotType::Boolean,
				InlineObject::True => SlotType::Boolean,
			}
		}
	}

	pub fn is_nullish(self) -> bool {
		match self.into_typed() {
			TypedSlot::InlineReference(reference) => match *reference {
				InlineObject::Undefined => true,
				InlineObject::Null => true,
				_ => false,
			}
			_ => false,
		}	
	}

	pub fn new_array(elements: Vec<Slot>) -> Self {
		ReferenceSlot::new_array(elements).into()
	}

	pub fn new_integer(i: isize) -> Self {
		if i < isize::MAX>>4 && i > isize::MIN>>4 {
			InlineNumberSlot::new(i as isize).into()
		} else {
			NumberSlot::new(i as i128).into()
		}
	}

	pub fn new_number_from_parts(i: [u64;2]) -> Self {
		NumberSlot::new((i[0] as i128) << 64 + (i[1] as i128)).into()
	}

	pub fn new_string(s: SharedString) -> Self {
		StringSlot::new(s).into()
	}

	pub fn new_object(names: Vec<SharedString>, inlines: Vec<Slot>) -> Self {
		ReferenceSlot::new_object(names, inlines).into()
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

	pub fn as_smi(&self) -> Option<i32> {
		match self.into_typed() {
			TypedSlot::InlineNumber(num) => {
				i32::try_from(*num).ok()
			},
			_ => None,
		}	
	}

	pub fn get_element(&mut self, index: i32) -> Option<&mut Slot> {
		if self.type_of() != SlotType::Object {
			return None
		}

		let obj = unsafe{transmute::<&mut Slot, &mut ReferenceSlot>(self)};

		obj.get_element(index)
	}

	pub fn get_property(&mut self, name: SharedString) -> Option<&mut Slot> {
		if self.type_of() != SlotType::Object {
			return None
		}

		let obj = unsafe{transmute::<&mut Slot, &mut ReferenceSlot>(self)};

		obj.get_property(name)
	}
}

impl PartialEq for Slot {
	fn eq(&self, other: &Self) -> bool {
		self.strict_equal_internal(other)
	}
}

impl LowerHex for Slot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:#x}", self.pointer.0 as usize)
	}
}