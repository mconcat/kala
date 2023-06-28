use crate::slot::Slot;

#[repr(transparent)]
pub struct Number(pub i128);

impl Number {
    pub fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        unimplemented!()
    }

    pub fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        unimplemented!("overflowing_sub")
    }
}