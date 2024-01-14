use super::slot::Slot;

#[derive(Clone)]
pub struct Array(pub Vec<Slot>);

impl Array {
    pub fn get_element(&mut self, index: usize) -> Option<&mut Slot> {
        self.0.get_mut(index)
    }
}