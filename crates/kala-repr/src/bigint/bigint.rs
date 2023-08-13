pub struct Bigint<Value: ?Sized> {
    pub sign: bool,
    pub value: Value,
}

