use super::{SlotTag, pointer::{full32, null32}, Slot};


pub const UNINITIALIZED: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const UNDEFINED: u64 = 0x0000_0000_FFFF_FFFF;
pub const FALSE: u64 = 0x0000_0000_0000_0000 | SlotTag::Constant as u64;
pub const TRUE: u64 = 0x0000_0001_0000_0000 | SlotTag::Constant as u64;

#[repr(C)]
pub struct UninitializedSlot {
    value: full32,
    pointer: full32,
}

#[repr(C)]
pub struct UndefinedSlot {
    value: null32,
    pointer: full32,
}

#[repr(C)]
pub struct BooleanSlot {
    value: u32,
    pointer: null32,
}

#[repr(C)]
pub union ConstantSlot {
    word: u64,
    pub uninitialized: UninitializedSlot,
    pub undefined: UndefinedSlot,
    pub boolean: BooleanSlot,
}

impl ConstantSlot {
    pub fn new_uninitialized() -> Self {
        Self {
            word: UNINITIALIZED,
        }
    }

    pub fn new_undefined() -> Self {
        Self {
            word: UNDEFINED,
        }
    }

    pub fn new_false() -> Self {
        Self {
            word: FALSE,
        }
    }

    pub fn new_true() -> Self {
        Self {
            word: TRUE,
        }
    }

    pub fn is_uninitialized(&self) -> bool {
        self.word == UNINITIALIZED
    }

    pub fn is_undefined(&self) -> bool {
        self.word == UNDEFINED
    }

    pub fn is_constant(&self) -> bool {
        self.word & 0x0000_0000_FFFF_FFFF == 0
    }

    pub fn is_falsy(&self) -> bool {
        self.word & 0xFFFF_FFFF_0000_0000 == 0
    }

    pub fn is_truthy(&self) -> bool {
        self.word & 0xFFFF_FFFF_0000_0000 != 0
    }
}

impl Into<Slot> for ConstantSlot {
	fn into(self) -> Slot {
        
	}	
}