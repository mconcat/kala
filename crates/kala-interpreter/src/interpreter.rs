use std::ops::{self, FromResidual};

use jessie_ast::{DeclarationIndex, CaptureDeclaration, LocalDeclaration, Variable, ParameterDeclaration, PropertyAccess, Expr};
use kala_repr::slot::Slot;

use crate::expression::eval_expr;

pub struct Frame {
    pub captures: Vec<(Variable, Slot)>,
    pub arguments: Vec<(ParameterDeclaration, Slot)>,
    pub locals: Vec<(LocalDeclaration, Slot)>,
}

impl Frame {
    pub fn get_capture(&mut self, index: u32) -> (&mut Variable, &mut Slot) {
        let (decl, slot) = self.captures.get_mut(index as usize).unwrap();
        (decl, slot)
    }

    pub fn get_argument(&mut self, index: u32) -> (&mut ParameterDeclaration, &mut Slot) {
        let (decl, slot) = self.arguments.get_mut(index as usize).unwrap();
        (decl, slot)
    }

    pub fn get_local(&mut self, index: u32) -> (&mut LocalDeclaration, &mut Slot) {
        let (decl, slot) = self.locals.get_mut(index as usize).unwrap();
        (decl, slot)
    }

    pub fn get_variable(&mut self, variable: Variable) -> &mut Slot {
        let mut var = match variable.declaration_index {
            DeclarationIndex::Capture(index) => self.get_capture(index),
            DeclarationIndex::Parameter(index) => self.get_argument(index),
            DeclarationIndex::Local(index) => self.get_local(index),
        };

        for access in variable.property_access {
            match access {
                PropertyAccess::Property(field) => {
                    var = var.get_property(field);
                }, 
                PropertyAccess::Element(index) => {
                    var = var.get_element(index);
                }
            }
        }

        var
    }
}

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

impl FromResidual for Evaluation {
    fn from_residual(residual: Self) -> Self {
        residual
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
    fn get_frame(&mut self) -> &mut Frame {
        self.frame.get_mut(self.frame.len()-1).unwrap()
    }
    
    pub fn initialize_local(&mut self, index: u32) -> Completion{
        let (decl, slot) = self.get_frame().get_local(i);
        let initial_value_expr = decl.get_initial_value().unwrap_or(Expr::undefined());
        let initial_value = eval_expr(self, &initial_value_expr)?;
        *slot = initial_value;
        Completion::Normal
    }
}