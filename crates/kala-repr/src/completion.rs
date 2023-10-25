use std::{convert::Infallible, ops::{self, FromResidual}};

use crate::slot::Slot;



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

impl ToString for Completion {
    fn to_string(&self) -> String {
        match self {
            Self::Normal => "undefined".to_string(),
            Self::Value(v) => v.to_string(),
            Self::Throw(v) => format!("Throw({})", v.to_string()),
            Self::Break => "Break".to_string(),
            Self::Continue => "Continue".to_string(),
            Self::Return(v) => format!("Return({})", v.to_string()),
            Self::ReturnEmpty => "ReturnEmpty".to_string(),
        }
    
    }
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
/* 
impl FromResidual<Evaluation> for Completion {
    fn from_residual(residual: Evaluation) -> Self {
        residual.into()
    }
}
*/
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
            Self::Throw(_) => ops::ControlFlow::Break(self),
            Self::Break => ops::ControlFlow::Break(self),
            Self::Continue => ops::ControlFlow::Break(self),
            Self::Return(_) => ops::ControlFlow::Break(self),
            Self::ReturnEmpty => ops::ControlFlow::Break(self),
        }
    }
}
/* 
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Evaluation {
    Value(Slot) = 1,
    Throw(Slot) = 2,
}
/* 
impl LowerHex for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "Value({:x})", v),
            Self::Throw(v) => write!(f, "Throw({:x})", v),
        }
    }
}
*/
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
            Self::Throw(_) => ops::ControlFlow::Break(self),
        }
    }
}

impl Into<Completion> for Evaluation {
    fn into(self) -> Completion {
        unsafe {std::mem::transmute(self)}
    }
}*/