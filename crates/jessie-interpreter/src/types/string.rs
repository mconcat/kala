use crate::{memory::memory::Pointer, slot::Slot};

pub struct String{
    str: Pointer<str>
}

impl From<Slot> for String {
    fn from(slot: Slot) -> Self {
        let ptr = slot.get_pointer();
        if ptr.is_null() {
            Self {
                str: Pointer::null()
            }
        } else {
            let ptr = unsafe { std::slice::from_raw_parts_mut(ptr, slot.value as usize) };
            Self {
                str: Pointer::new(ptr)
            }
        }
    }
}