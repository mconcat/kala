use std::rc::Rc;




pub enum Value {
    // Special value for TDZ variables
    Uninitialized,

    // Primitive Values
    // Undefined
    Undefined,

    // Null
    Null,




    // String
    ShortString([u8; 16]), // max length 16bytes
    LongString(Box<Vec<u64>>), // right padded \0
    

    // Object kinds: https://docs.rs/Boa/latest/src/boa/object/mod.rs.html#92

    //////// Reference Values

    // Array
    // Follows naming of {IndexType|Typed}{ElementType}Array
    // Indicies are u32 type
    Array(Array),



    // StringIterator
    // TODO

    // ArrayIterator
    // TODO




    // Map
    // TODO

    // MapIterator
    // TODO

    // RegExp
    // Maybe not supported

    // ForInIterator
    // TODO

    // Function
    Function(Function),

    // Set
    // TODO

    // SetIterator
    // TODO

    // Error
    Error(),

    // Ordinary
    // Ordinary(), // didnt understand this one

    // Date
    // Todo...?

    // NativeObject
    NativeObject(Box<dyn NativeObject>),

    Promise(),

    // ArrayBuffer(Box<Vec<u8>>),
    // DataView(),
}

