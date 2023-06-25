pub trait UnsafeInto<T> {
    unsafe fn unsafe_into(self) -> T;
}