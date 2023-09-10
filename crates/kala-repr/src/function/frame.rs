use std::{rc::Rc, cell::RefCell};

use crate::slot::Slot;

//#[derive(Clone)]
//pub struct Variable(pub Rc<RefCell<Slot>>);
/* 
impl Variable {
    pub fn uninitialized() -> Self {
        Self(Rc::new(RefCell::new(Slot::new_uninitalized())))
    }
}
*/


/*
0    1    2    3    4    5    6    7    
|----|----|----|----|----|----|----|----|
-3   -2   -1   0    1    2    3    4
P1   P0   C0   L0  L1   L2   L3   L4
               ^ 
               fp=3
*/    

pub struct Frame {
    pub fp: usize,
    pub capture_count: usize, // TODO: this will be statically known and DeclarationIndex will take account of this
    pub stack: *mut Vec<Slot>,
}

impl Frame {
    pub fn new(fp: usize, capture_count: usize, stack: *mut Vec<Slot>) -> Self {
        Self {
            fp,
            capture_count,
            stack,
        }
    }

    pub fn get_capture(&mut self, index: usize) -> Option<&mut Slot> {
        let stack = unsafe{&mut*self.stack};
        stack.get_mut(self.fp-index-1)
    }

    pub fn get_argument(&mut self, index: usize) -> Option<&mut Slot> {
        let stack = unsafe{&mut*self.stack};
        stack.get_mut(self.fp-self.capture_count-index-1)

    }

    pub fn get_local(&mut self, index: usize) -> Option<&mut Slot> {
        let stack = unsafe{&mut*self.stack};
        stack.get_mut(index-self.fp)
    }
}
