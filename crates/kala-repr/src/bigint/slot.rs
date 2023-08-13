use std::ops::Deref;

use crate::slot::{Slot, SlotTag};

use super::Bigint;

#[repr(C)]
pub struct BigintSlot {
	sign_len: i32,
	pointer: u32, // *mut [u64]
}

impl Into<Slot> for BigintSlot {
	fn into(self) -> Slot {
		Slot(unsafe{std::mem::transmute::<Self, u64>(self)} | SlotTag::Bigint as u64)
	}	
}