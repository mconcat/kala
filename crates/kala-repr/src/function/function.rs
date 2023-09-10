use utils::SharedString;

use crate::slot::Slot;

use super::{Frame};

pub trait Executable {
    type State;

    fn execute(&self, state: &mut Self::State, frame: &mut Frame);
    fn local_size(&self) -> usize;
}

#[derive(Clone)]
pub struct FunctionPrototype<Code> {
    pub name: Option<SharedString>,
    pub code: Code,
    pub captures: Vec<Slot>,
}

impl<Code> FunctionPrototype<Code> {
    pub fn new(name: Option<SharedString>, code: Code, captures: Vec<Slot>) -> Self {
        Self {
            name,
            code,
            captures,
        }
    }
}

impl<Code: Executable> FunctionPrototype<Code> {
    pub fn call(&self, state: &mut Code::State, stack_ptr: *mut Vec<Slot>) {
        let stack = unsafe{&mut *stack_ptr};

        // assuming parametes are already pushed to the stack
        stack.append(&mut self.captures.clone());
        let fp = stack.len();
        stack.append(&mut vec![Slot::new_uninitalized(); self.code.local_size()]);
        let mut frame = Frame::new(fp, self.captures.len(), stack);
        self.code.execute(state, &mut frame);
    }
}