use utils::SharedString;

use super::{Frame, Variable};

pub trait Executable {
    type State;

    fn execute(&self, state: &mut Self::State, frame: &mut Frame);
    fn local_size(&self) -> usize;
}

#[derive(Clone)]
pub struct FunctionPrototype<Code> {
    pub name: SharedString,
    pub code: Code,
    pub captures: Vec<Variable>,
}

impl<Code> FunctionPrototype<Code> {
    pub fn new(name: SharedString, code: Code, captures: Vec<Variable>) -> Self {
        Self {
            name,
            code,
            captures,
        }
    }
}

impl<Code: Executable> FunctionPrototype<Code> {
    pub fn call(&self, state: &mut Code::State, stack_ptr: *mut Vec<Variable>) {
        let stack = unsafe{&mut *stack_ptr};

        // assuming parametes are already pushed to the stack
        stack.append(&mut self.captures.clone());
        let fp = stack.len();
        stack.append(&mut vec![Variable::uninitialized(); self.code.local_size()]);
        let mut frame = Frame::new(fp, self.captures.len(), stack);
        self.code.execute(state, &mut frame);
    }
}