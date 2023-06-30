use core::{slice};
use std::{ops::{Add, Sub, Mul, Div, Deref}};

use super::pointer::{np32, wp64, null32};

#[repr(u64)]
pub enum SlotTag {
	// Values that can (possibly) be stored in heap
	Reference = 0b_0000_0000,
	Number = 0b_0000_0010,
	String = 0b_0000_0100,
	Bigint = 0b_0000_0110,

	// Values that can only be stored inlined(pointer is always null)
	Uninitialized = 0b_0000_0001,
	Undefined = 0b_0000_0011,
	_Reserved = 0b_0000_0101,
	Constant = 0b_0000_0111,
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
pub struct SlotPointer(u64);

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

#[repr(C)]
pub struct ReferenceHeader {
	reference_count: u32,
	len: u32,
}

#[repr(u64)]
pub enum TypedSlot {
	Reference(ReferenceSlot) = SlotTag::Reference as u64,
	Number(NumberSlot) = SlotTag::Number as u64,
	Bigint(BigintSlot) = SlotTag::Bigint as u64,
	String(StringSlot) = SlotTag::String as u64,
	Constant(ConstantSlot) = SlotTag::Constant as u64,
	Undefined(UndefinedSlot) = SlotTag::Undefined as u64,
	Uninitialized(UninitializedSlot) = SlotTag::Uninitialized as u64,
}

#[repr(C)]
pub struct ReferenceSlot {
	value: u32,
	pointer: np32<Slot>,
}

impl Deref for ReferenceSlot {
	type Target = (ReferenceHeader, *const [Slot]);

	fn deref(&self) -> &Self::Target {
		if self.pointer.is_null() {
			panic!("ReferenceSlot is null")
		} else {
			let header = unsafe {*(self.pointer.as_raw_pointer() as *const ReferenceHeader)};
			let len = header.len as usize;
			const header_size: usize = std::mem::size_of::<ReferenceHeader>() / std::mem::size_of::<Slot>(); // assumes both are aligned to 8 bytes
			let slice = unsafe{ slice::from_raw_parts(self.pointer.as_raw_pointer().add(header_size), len - header_size) };
			&(header, slice)
		}
	}
}

impl Into<Slot> for ReferenceSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Reference as u64) }
	}	
}

#[repr(C)]
pub struct NumberSlot {
	value: i32,
	pointer: np32<i128>,
}

impl NumberSlot {
	pub fn new(value: i128) -> Self {
		Self{ value: 0, pointer: np32::new(value) }
	}

	pub fn new_inline(value: i32) -> Self {
		Self{ value, pointer: np32::null() }
	}

	pub fn new_positive_infinity() -> Self {
		Self{ value: 0, pointer: np32::new(i128::MAX) }
	}

	pub fn new_negative_infinity() -> Self {
		Self{ value: 0, pointer: np32::new(i128::MIN) }
	}
}

impl Into<Slot> for NumberSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute<Self, u64>(self)} | SlotTag::Number) }
	}	
}

impl Deref for NumberSlot {
	type Target = i128;

	fn deref(&self) -> &Self::Target {
		if self.pointer.is_null() {
			&i128::from(self.value)
		} else {
			unsafe {&*self.pointer}
		}
	}
}

impl Add for NumberSlot {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_add(rhs.value);
			if overflow {
				return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let (res, overflow) = self_value.overflowing_add(rhs_value);
		if overflow {
			if res < 0 {
				return NumberSlot::new_positive_infinity()
			} else {
				return NumberSlot::new_negative_infinity()
			}
		}

		NumberSlot::new(res)
	}
}

impl Add for &NumberSlot {
	type Output = NumberSlot;

	fn add(self, rhs: Self) -> Self::Output {
		*self + *rhs
	}
}

impl Sub for NumberSlot {
	type Output = Self; 

	fn sub(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_sub(rhs.value);
			if overflow {
				//return NumberSlot::new(res as i128)
				unimplemented!("overflow")
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let (res, overflow) = self_value.overflowing_sub(rhs_value);
		if overflow {
			if res < 0 {
				return NumberSlot::new_positive_infinity()
			} else {
				return NumberSlot::new_negative_infinity()
			}
		}

		NumberSlot::new(res)
	}
}

impl Mul for NumberSlot {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			let (res, overflow) = self.value.overflowing_mul(rhs.value);
			if overflow {
				unimplemented!("overflow")
				//return NumberSlot::new(res as i128)
			}
			return NumberSlot::new_inline(res) 
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		let self_lo = self_value & 0xFFFF_FFFF_FFFF_FFFF;
		let self_hi = self_value >> 64;
		let rhs_lo = rhs_value & 0xFFFF_FFFF_FFFF_FFFF;
		let rhs_hi = rhs_value >> 64;

		let mut hihi = self_hi * rhs_hi;
		if hihi>>64 != 0 {
			unimplemented!("overflow")
		}

		let lolo= self_lo * rhs_lo;
		let lohi = self_hi * rhs_lo;
		let hilo = self_lo * rhs_hi;

		let mut hihi_carry = false;
		let (mut mid, mut mid_carry) = lohi.overflowing_add(hilo);
		if mid_carry {
			(hihi, hihi_carry) = hihi.overflowing_add(1);
			if hihi>>64 != 0 || hihi_carry {
				unimplemented!("overflow")
			}
		}
		(mid, mid_carry) = mid.overflowing_add(lolo>>64);
		if mid_carry {
			(hihi, hihi_carry) = hihi.overflowing_add(1);
			if hihi>>64 != 0 || hihi_carry {
				unimplemented!("overflow")
			}
		}

		NumberSlot::new(mid)
	}
}

impl Div for NumberSlot {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		if self.pointer.is_null() && rhs.pointer.is_null() {
			if rhs.value == 0 {
				unimplemented!("divide by zero, TODO: throw")
			}

			if self.value % rhs.value == 0 {
				let (res, overflow) = self.value.overflowing_div(rhs.value);
				if overflow {
					unimplemented!("overflow")
				}
				return NumberSlot::new_inline(res)
			}

			// else, fallthrough
		}

		let self_value = if self.pointer.is_null() {
			self.value as i128
		} else {
			*self.pointer
		};

		let rhs_value = if rhs.pointer.is_null() {
			rhs.value as i128
		} else {
			*rhs.pointer
		};

		unimplemented!("long division")
	}
}

#[repr(C)]
pub union BigintSlot {
	sign: i64, 
	pointer: wp64<u64>,
}

impl Into<Slot> for BigintSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Bigint) }
	}	
}


#[repr(C)]
pub struct StringSlot {
	pointer: wp64<u8>,
}

impl Into<Slot> for StringSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::String) }
	}	
}

impl Deref for StringSlot {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		unsafe{std::str::from_utf8_unchecked(&self.pointer)}
	}
}

#[repr(u32)]
pub enum ConstantValue {
	False = 0,
	True = 1,
}

#[repr(C)]
pub struct ConstantSlot {
	value: ConstantValue,
	pointer: null32,
}

impl Into<Slot> for ConstantSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Constant) }
	}	
}

impl Deref for ConstantSlot {
	type Target = ConstantValue;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

#[repr(C)]
pub struct UndefinedSlot {
	value: null32,
	pointer: null32,
}

impl Into<Slot> for UndefinedSlot {
	fn into(self) -> Slot {
		Slot{ pointer: SlotPointer(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Undefined) }
	}	
}

#[repr(C)]
pub struct UninitializedSlot {
	value: null32,
	pointer: null32,
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