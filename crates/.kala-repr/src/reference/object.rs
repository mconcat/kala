use crate::{slot::Slot, number::NumberObject, bigint::BigintObject, string::object::StringObject, constant::primitive::PrimitiveConstant, object::Object};

// Language primitive objects that cannot have key-value properties
// TODO: enum discriminants could be tagged, reduce Reference size to 3 words
pub enum Reference {
    // Primitive constants are stored as a ConstantSlot by default,
    // and promoted to a Reference when captured by a closure,
    // or escape the function scope in any other way.
    PrimitiveConstant(PrimitiveConstant),
    
    // unlike other immut references, the size of number is not dependent on the machine word size, but fixed at 16 bytes
    Number(NumberObject), // Fixed128, immutable
    Bigint(BigintObject), // immutable
    String(StringObject), // immutable

    Function(Function), // immutable
    Array(Vec<Slot>),

    Object(Object),

    /* 
    TypedUint8Array(Box<Vec<u8>>),
    TypedInt8Array(Box<Vec<i8>>),
    TypedUint16Array(Box<Vec<u16>>),
    TypedInt16Array(Box<Vec<i16>>),
    TypedUint32Array(Box<Vec<u32>>),
    TypedInt32Array(Box<Vec<i32>>),
    TypedFloat64Array(Box<Vec<i128>>), // Fixed128
    TypedBigInt64Array(Box<Vec<i64>>),
    TypedBigUint64Array(Box<Vec<u64>>),
    */
}