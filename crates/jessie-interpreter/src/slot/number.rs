pub fn promote_i64(i: i64) -> i128 {
    (i as i128) << 64
}

pub fn promote_i32(i: i32) -> i128 {
    (i as i128) << 64
}

pub fn bound_i32(i: i64) -> bool {
    i >= i32::MIN as i64 && i <= i32::MAX as i64
}