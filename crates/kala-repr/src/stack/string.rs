use crate::{Slot, pointer::wp64, SlotTag, SlotPointer};

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
