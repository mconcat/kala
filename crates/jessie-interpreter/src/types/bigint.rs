pub enum Bigint {
    // Bigint
    I64(i64),
    I128([u64; 2]),
    Dynamic(Box<Vec<u64>>),
}