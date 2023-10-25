use std::{cell::{Cell, RefCell, UnsafeCell}, rc::{UniqueRc, Weak, Rc}, mem::{replace, zeroed}};

use utils::SharedString;

use crate::completion::Completion;

use super::slot::Slot;

pub struct Function {
    pub name: Option<SharedString>,
    //pub parameters_len: usize,
    // pub captures: Vec<Slot>,
    //pub locals_len: usize,
    pub function: Box<dyn Fn(&mut Frame, Vec<Slot>) -> Completion>,
}

// Frame points to the slice of the stack
// indicides are relative to fp
// arguments are reversed pushed to the stack, -argument_len..-1
// 0 is reserved
// captures, 1..1+captures_len
// locals, 1+captures_len+1..1+captures_len+locals_len
#[derive(Debug)]
pub struct Frame {
    pub slots: Vec<Slot>,
    pub fp: usize,
    pub captures: usize,
}

impl Frame {
    pub fn empty() -> Self {
        Self {
            slots: vec![],
            fp: 0,
            captures: 0,
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::empty()
    }
}

pub struct FrameRecovery {
    pub fp: usize,
    pub captures: usize,
}

impl Frame {
    pub fn get_argument(&mut self, index: usize) -> &mut Slot {  ㅕu
        &mut self.slots[self.fp - 1 - index]
    }

    pub fn get_capture(&mut self, index: usize) -> &mut Slot {
        &mut self.slots[self.fp + 1 + index]
    }

    pub fn get_local(&mut self, index: usize) -> &mut Slot {
        println!("getting local {:?} {}", self, index);
        &mut self.slots[self.fp + 1 + self.captures + index]
    }


    // destructs the current frame and returns the child frame
    pub fn enter_function_frame(&mut self, captures: Vec<Slot>, local_len: usize) -> FrameRecovery {
        // constructing child frame
        println!("entering function frame");
        let fp = self.slots.len();
        self.slots.extend(vec![Slot::UNINITIALIZED]); // reserved 0 index
        let captures_len = captures.len();
        self.slots.extend(captures);
        self.slots.extend(vec![Slot::UNINITIALIZED; local_len]);

        let recovery = FrameRecovery {
            fp: replace(&mut self.fp, fp),
            captures: replace(&mut self.captures, captures_len),

        };
    
        recovery
    }

    // destructs the child frame and recovers the parent frame
    pub fn exit_function_frame(&mut self, recovery: FrameRecovery) {
        self.slots.truncate(recovery.fp);

        replace(&mut self.fp, recovery.fp);
        replace(&mut self.captures, recovery.captures);
    }
}

// parameter-capture-0-local
pub struct Stack {
    pub slots: Vec<Slot>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
        }
    }


    /* 
    pub fn get_variable(&mut self, index: isize) -> &mut Slot {
        let index = ((self.fp() as isize) + index) as usize;
        &mut self.slots[index]
    }
    */
}


