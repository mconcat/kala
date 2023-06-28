use crate::slot::Slot;

pub struct Boolean(bool);

impl From<Slot> for Boolean {
    fn from(slot: Slot) -> Self {
        Boolean(slot.value != 0)
    }
}