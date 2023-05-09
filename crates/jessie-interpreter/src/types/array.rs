struct ArrayInternal<T: Sized> {
    object: Option<Object>,
    array: Vec<T>,
}

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
}