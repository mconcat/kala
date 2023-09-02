use std::{ops::{self, FromResidual}, convert::Infallible, fmt::{LowerHex, self}};

use kala_repr::{slot::Slot, function::Frame};

use crate::expression::eval_expr;


#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Completion {
    Normal = 0, // Normal completion, execution continues without inturrupt
    Value(Slot) = 1, // evaluation result of an expression, discarded when treated as a statement
    Throw(Slot) = 2, // Throw, unwinds the execution until the innermost try-catch
    Break = 3, // Break, unwinds the execution until the innermost loop. 
    Continue = 4,
    Return(Slot) = 5, // Return, unwinds the execution until the innermost function call
    ReturnEmpty = 6,
}

impl FromResidual for Completion {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}

impl FromResidual<Option<Infallible>> for Completion {
    fn from_residual(residual: Option<Infallible>) -> Self {
        match residual {
            None => Self::Normal,
            Some(_) => unreachable!("Infallible")
        }
    }
}

impl FromResidual<Evaluation> for Completion {
    fn from_residual(residual: Evaluation) -> Self {
        residual.into()
    }
}

impl ops::Try for Completion {
    type Output = Slot;

    type Residual = Self;

    fn from_output(output: Self::Output) -> Self {
        Self::Value(output)
    }

    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Normal => ops::ControlFlow::Continue(Slot::new_undefined()),
            Self::Value(v) => ops::ControlFlow::Continue(v),
            Self::Throw(v) => ops::ControlFlow::Break(self),
            Self::Break => ops::ControlFlow::Break(self),
            Self::Continue => ops::ControlFlow::Break(self),
            Self::Return(v) => ops::ControlFlow::Break(self),
            Self::ReturnEmpty => ops::ControlFlow::Break(self),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Evaluation {
    Value(Slot) = 1,
    Throw(Slot) = 2,
}

impl LowerHex for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "Value({:x})", v),
            Self::Throw(v) => write!(f, "Throw({:x})", v),
        }
    }
}

impl ToString for Evaluation {
    fn to_string(&self) -> String {
        match self {
            Self::Value(v) => format!("Value({})", v.to_string()),
            Self::Throw(v) => format!("Throw({})", v.to_string()),
        }
    }
}

impl ToString for &Evaluation {
    fn to_string(&self) -> String {
        match self {
            Evaluation::Value(v) => format!("Value({})", v.to_string()),
            Evaluation::Throw(v) => format!("Throw({})", v.to_string()),
        }
    }
}

impl FromResidual for Evaluation {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}

impl FromResidual<Option<Infallible>> for Evaluation {
    fn from_residual(residual: Option<Infallible>) -> Self {
        match residual {
            None => Self::Throw(Slot::new_undefined()),
            Some(_) => unreachable!("Infallible")
        }
    }
}

impl ops::Try for Evaluation {
    type Output = Slot;
    type Residual = Self;

    fn from_output(output: Self::Output) -> Self {
        Self::Value(output)
    }

    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Value(v) => ops::ControlFlow::Continue(v),
            Self::Throw(v) => ops::ControlFlow::Break(self),
        }
    }
}

impl Into<Completion> for Evaluation {
    fn into(self) -> Completion {
        unsafe {std::mem::transmute(self)}
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
    pub frame: Vec<Frame>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            frame: vec![Frame::new()]
        }
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        self.frame.last_mut().unwrap()
    }


    fn get_local_initial_value(&mut self, index: u32) -> Evaluation {
        let decl = self.get_frame().locals.get(index as usize)?.0.clone();
        let initial_value_expr = decl.get_initial_value();
        match initial_value_expr {
            None => Evaluation::Value(Slot::new_undefined()),
            Some(initial_value_expr) => eval_expr(self, initial_value_expr)
        }
    }
    
    pub fn initialize_local(&mut self, index: u32) -> Completion {
        let initial_value = self.get_local_initial_value(index)?;
        let slot = &mut self.get_frame().locals.get_mut(index as usize)?.1;
        *slot = initial_value;
        Completion::Normal
    }
}