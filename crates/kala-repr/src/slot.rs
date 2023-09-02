use std::{mem::transmute, fmt::LowerHex, sync::Once};

use utils::SharedString;

use crate::{reference::ReferenceSlot, number::NumberSlot, bigint::BigintSlot, string::slot::StringSlot, memory::alloc::Ref, function::Variable};

use std::sync::LazyLock;

// unwrapping any Slot with this value will throw an error
// must be manually checked in all cases of unwrapping
pub const UNINITIALIZED: Slot = Slot {
	value: isize::MIN, // all bits set to 1
	pointer: Ref::null(SlotTag::Reference),
};

// has type tag of Reference, so typeof NULL == typeof Reference
// pub static NULL_LOCK: LazyLock<Slot> = LazyLock::new(|| Slot::new_reference(Object::new(Vec::new())));

fn NULL() -> Slot {
    use std::mem::MaybeUninit;
    
    static mut NULL: MaybeUninit<Slot> = MaybeUninit::uninit();
    static mut ONCE: Once = Once::new();
    
    unsafe {
        ONCE.call_once(|| NULL = MaybeUninit::new(Slot::new_object(Vec::new(), Vec::new())));
        
        NULL.as_ptr().read()
    }
}

pub const UNDEFINED: Slot = Slot {
	value: 0x0000_0000,
	pointer: Ref::null(SlotTag::Reference),
};

pub const FALSE: Slot = Slot {
	value: 0x0000_0000,
	pointer: Ref::null(SlotTag::String),
};

pub const TRUE: Slot = Slot {
	value: 0x0000_0001,
	pointer: Ref::null(SlotTag::String),
};

pub const ZERO_NUMBER: Slot = Slot {
	value: 0x0000_0000,
	pointer: Ref::null(SlotTag::Number),
};

pub const ZERO_BIGINT: Slot = Slot {
	value: 0x0000_0000,
	pointer: Ref::null(SlotTag::Bigint),
};

// pub const EMPTY_STRING: LazyLock<Slot> = LazyLock::new(|| Slot::new_string(&[]));

// pub const ZERO_NUMBER: Slot = Slot(0x0000_0000_0000_0001);

// pub const ZERO_BIGINT: Slot = Slot(0x0000_0000_0000_0003);


#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
pub enum SlotTag {
	Reference = 0b_0000_0000,
	Number = 0b_0000_0001,
	String = 0b_0000_0010,
	Bigint = 0b_0000_0011,

	/*
	ConstReference = 0b0000_0100,
	ConstNumber = 0b0000_0101,
	ConstString = 0b0000_0110,
	ConstBigint = 0b0000_0111,
	*/

	Uninitialized = 0b_0000_1000,
}

impl SlotTag {
	pub fn into_type(self) -> SlotType {
		unsafe{transmute(self as u64 & 0b_0000_0011)}
	}

	pub fn is_mutable(self) -> bool {
		(self as u64 & 0b_0000_0100) == 0
	}
	
	pub fn is_uninitialized(self) -> bool {
		self == SlotTag::Uninitialized
	}
}

#[repr(usize)]
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

#[repr(usize)]
pub enum TypedSlot {
	Reference(ReferenceSlot) = 0b_0000_0000,
	Number(NumberSlot) = 0b_0000_0001,
	String(StringSlot) = 0b_0000_0010,
	Bigint(BigintSlot) = 0b_0000_0011,

	Uninitialized = 0b_0000_1000,
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

const SLOT_TAG_MASK: usize = 0b_0000_0000_0000_0000_0000_0000_0000_0011;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Slot {
	pub(crate) value: isize,
	pub(crate) pointer: Ref<()>,
}

impl Slot {
	pub fn raw_equal(&self, other: &Self) -> bool {
		self.value == other.value && self.pointer.0 == other.pointer.0
	}

	pub fn tag(&self) -> SlotTag {
		if self.is_uninitialized() {
			return SlotTag::Uninitialized
		}

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
	pub fn into_typed(&self) -> TypedSlot {
		if self.is_uninitialized() {
			return TypedSlot::Uninitialized
		}

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
		match self.tag() {
			SlotTag::Reference => {
				if self.raw_equal(&NULL()) || self.raw_equal(&UNDEFINED) {
					false
				} else {
					true
				}
			}
			SlotTag::Number => {
				if self.raw_equal(&ZERO_NUMBER) {
					// zero
					false
				} else {
					true
				}
			},
			SlotTag::String => {
				if self.value == 0 {
					// false or empty string
					false
				} else {
					true
				}
			},
			SlotTag::Bigint => {
				if self.raw_equal(&ZERO_BIGINT) {
					// zero
					false
				} else {
					true
				}
			},
			SlotTag::Uninitialized => panic!("is_falsy on uninitialized slot"),
		}
	}

	pub fn is_truthy(&self) -> bool {
		!self.is_falsy()
	}

	pub fn is_undefined(&self) -> bool {
		self.raw_equal(&UNDEFINED)	
	}

	pub fn is_null(&self) -> bool {
		false // TODO
	}

	pub fn is_true(&self) -> bool {
		self.raw_equal(&TRUE)	
	}

	pub fn is_false(&self) -> bool {
		self.raw_equal(&FALSE)
	}

	pub fn type_of(&self) -> SlotType {
		if self.is_uninitialized() {
			panic!("uninitialized slot")
		}

		let ty: SlotType = unsafe{transmute(self.tag())};

		if ty == SlotType::String {
			if self.pointer.is_null() {
				return SlotType::Boolean
			} else {
				return SlotType::String
			}
		}

		if ty == SlotType::Object {
			if self.is_undefined() {
				return SlotType::Undefined
			} else {
				return SlotType::Object
			}
		}

		ty
	}

	pub fn is_nullish(self) -> bool {
		return self == UNDEFINED || self == NULL()
	}

	pub fn new_array(elements: Vec<Slot>) -> Self {
		ReferenceSlot::new_array(elements).into()
	}

	pub fn new_integer(i: usize) -> Self {
		if i <= isize::MAX as usize {
			NumberSlot::new_inline(i as isize).into()
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

	pub fn new_function<Code>(name: SharedString, code: Code, captures: Vec<Variable>) -> Self {
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
			TypedSlot::Number(num) => Into::<i128>::into(num).try_into().ok(),
			_ => None,
		}	
	}

	pub fn get_element(&mut self, index: i32) -> Option<Slot> {
		let mut obj = self.as_reference()?;
		obj.get_element(index)
	}

	pub fn get_property(&self, name: SharedString) -> Option<Slot> {
		let mut obj = self.as_reference()?;
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
		write!(f, "{:#x}:{:#x}", self.value, self.pointer.0 as usize)
	}
}