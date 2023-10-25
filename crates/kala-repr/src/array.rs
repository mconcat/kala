use super::slot::Slot;

pub struct Array(pub Vec<Slot>);

impl Array {
    pub fn get_element(&mut self, index: usize) -> Option<&mut Slot> {
        self.0.get_mut(index)
    }
}