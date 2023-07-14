use std::ops;

use kala_repr::Slot;

pub struct Frame {
    pub captures: Vec<Slot>,
    pub arguments: Vec<Slot>,
    pub locals: Vec<Slot>,
}

impl Frame {
    pub fn get_capture(&mut self, index: usize) -> &mut Slot {
        self.captures.get_mut(index).unwrap()
    }

    pub fn get_argument(&mut self, index: usize) -> &mut Slot {
        self.arguments.get_mut(index).unwrap()
    }

    pub fn get_local(&mut self, index: usize) -> &mut Slot {
        self.locals.get_mut(index).unwrap()
    }

    pub fn get_variable(&mut self, variable: Variable) -> &mut Slot {
        let mut var = match variable {
            Variable::Capture(index) => self.get_capture(index),
            Variable::Parameter(index) => self.get_argument(index),
            Variable::Local(index) => self.get_local(index),
        };

        for access in variable.property_access.0 {
            match access {
                PropertyAccess::Property(field) => {
                    var = var.get_property(field);
                }, 
                PropertyAccess::Element(index) => {
                    var = var.get_element(index);
                }
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum ThrowCompletion {
    Normal = 0,
    Throw(Slot) = 2,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Completion {
    Normal = 0, // Normal completion, execution continues without inturrupt
    Value(Slot) = 1, // evaluation result of an expression
    Throw(Slot) = 2, // Throw, unwinds the execution until the innermost try-catch
    Break = 3, // Break, unwinds the execution until the innermost loop
    Continue = 4,
    Return(Slot) = 5, // Return, unwinds the execution until the innermost function call
    ReturnEmpty = 6,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Evaluation {
    Value(Slot) = 1,
    Throw(Slot) = 2,
}

impl ops::Try for Evaluation {
    type Ok = Slot;
    type Error = Slot;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Value(v) => Ok(v),
            Self::Throw(v) => Err(v),
        }
    }
}

impl ops::Try for ThrowCompletion {
    type Ok = ();
    type Error = Slot;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            ThrowCompletion::Normal => Ok(()),
            ThrowCompletion::Throw(v) => Err(v),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        Self::Throw(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        Self::Normal
    }
}

impl ops::Try for Completion {
    type Ok = ();
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Completion::Normal => Ok(()),
            Completion::Value(_) => Ok(()),
            _ => Err(self),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        v
    }

    fn from_ok(v: Self::Ok) -> Self {
        Self::Normal
    }
}

impl Into<Completion> for ThrowCompletion {
    fn into(self) -> Completion {
        unsafe{std::mem::transmute(self)}
    }
}

impl Into<Completion> for Evaluation {
    fn into(self) -> Completion {
        unsafe{std::mem::transmute(self)}
    }
}
/* 
#[derive(Debug, PartialEq, Clone)]
pub struct BlockFlag(pub u32);

impl BlockFlag {
    pub fn new() -> Self {
        BlockFlag(0)
    }

    // returns true except for the global scope
    // return is allowed
    pub fn within_function(&self) -> bool {
        self.0 & 0b0001 == 1
    }

    // for or while
    // continue and break are allowed
    pub fn within_loop(&self) -> bool {
        self.0 & 0b0010 == 1
    }

    // try block
    // if true, throw will be caught by the catch block
    // if false, throw will invoke rust side panic
    pub fn within_try(&self) -> bool {
        self.0 & 0b0100 == 1
    }

    // switch block
    // break is obligatory at the end of each case per jessie spec
    pub fn within_switch(&self) -> bool {
        self.0 & 0b1000 == 1
    }

    pub fn set_within_function(&mut self) {
        self.0 |= 0b0001;
    }

    pub fn set_within_loop(&mut self) {
        self.0 |= 0b0010;
    }

    pub fn set_within_try(&mut self) {
        self.0 |= 0b0100;
    }

    pub fn set_within_switch(&mut self) {
        self.0 |= 0b1000;
    }

    pub fn clear_within_function(&mut self) {
        self.0 &= 0b1110;
    }

    pub fn clear_within_loop(&mut self) {
        self.0 &= 0b1101;
    }

    pub fn clear_within_try(&mut self) {
        self.0 &= 0b1011;
    }

    pub fn clear_within_switch(&mut self) {
        self.0 &= 0b0111;
    }
}
*/
pub struct Interpreter {
    pub frame: Frame,
}
