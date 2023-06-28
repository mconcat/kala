use crate::{memory::memory::Pointer, slot::Slot};

pub struct Array {
    arr: Pointer<[Slot]>
}

impl From<Slot> for Array {
    fn from(slot: Slot) -> Self {
        let arr = unsafe {std::slice::from_raw_parts_mut(slot.get_slot_pointer(), slot.value.try_into().unwrap())};
        Self {
            arr: Pointer::new(arr),
        }
    }
}



/* 
pub enum Array {
    Int64Array(Box<ArrayInternal<i64>>),
    NumberArray(Box<ArrayInternal<[u64; 2]>>),
    Bigint64Array(Box<ArrayInternal<i64>>),
    Bigint128Array(Box<ArrayInternal<[u64; 2]>>),
    BigintDynamicArray(Box<ArrayInternal<Vec<u64>>>),
    ShortStringArray(Box<ArrayInternal<[u8; 16]>>),
    StringArray(Box<ArrayInternal<Vec<u64>>>),
    ValueArray(Box<ArrayInternal<Value>>),

    DictionaryArray(Box<ArrayInternal<Value>>), // HashMap<u32, Value>

    ObjectArray(Box<ArrayInternal<Value>>), // HashMap<String, Value>, basically object but still inherits Array prototype
}*/