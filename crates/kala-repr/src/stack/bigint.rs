use crate::{pointer::wp64, Slot, SlotPointer, SlotTag};

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
