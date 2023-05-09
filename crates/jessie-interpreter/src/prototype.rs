// The set of language native prototype. Not allowed to extend in the runtime
pub enum Prototype {
    Object,
    Function,
    Array,
    TypedArray,
    String,
    Number,
    Bigint,
    Boolean,
    Date,
    Error,
    Promise,
    // Map, Set,
    // ArrayBuffer,
    // DataView,
}
