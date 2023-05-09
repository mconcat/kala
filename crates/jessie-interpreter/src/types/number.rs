pub enum Number {
    // Number
    Int64(i64), // follows js integer range
    Fixed128([u64; 2]), // 64.64, we subst float64 for fixed128
}