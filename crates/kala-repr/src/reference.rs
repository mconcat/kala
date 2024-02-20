use core::fmt;
use std::{any::Any, ops::{Index, IndexMut}, ptr, cell::{Ref, RefCell}, mem::ManuallyDrop, fmt::{Formatter, Debug}, rc::Rc};

use crate::{number::NUMBER_ZERO, object::Property, slot::Slot, function::Function, error::Error, completion::Completion};

use super::{number::Number, object::Object, constant::Constant, array::Array};

#[derive(Clone)]
pub enum Reference {
    Object(Object),
    // Integer(Integer),
    Number(Number),
    Constant(Constant),
    String(Rc<str>),
    Array(Array),
    
    //StaticFunction(StaticFunction<Box<dyn Any>>), // no capture, just function pointer
    Function(Function), // closure with captured environment variable

    Error(Error),

    NativeFunction(Rc<str>, Rc<RefCell<dyn FnMut(&mut [Slot]) -> Completion>>),

    // Reference type for accessor property
}

impl Debug for Reference {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Reference::Object(object) => {
                /* 
                write!(f, "{{")?;
                for prop in &object.properties {
                    write!(f, "{:?}, ", prop)?;
                }
                write!(f, "}}")  
                */
                write!(f, "[Object]")
            }
            Reference::Number(number) => write!(f, "Number({:?})", number),
            Reference::Constant(constant) => write!(f, "Constant({:?})", constant),
            Reference::String(string) => write!(f, "String({:?})", string),
            Reference::Array(array) => write!(f, "[Array]"),
            Reference::Function(function) => write!(f, "[Function {:?}]", function.name),
            Reference::Error(error) => write!(f, "[Error]"),
            Reference::NativeFunction(name, _) => write!(f, "[NativeFunction {:?}]", name),
        }
    }

}

impl Reference {
    pub fn is_falsy(&self) -> bool {
        match self {
            Reference::Object(object) => object.is_empty(),
            Reference::Number(number) => number == &NUMBER_ZERO,
            Reference::Constant(constant) => constant.is_falsy(),
            Reference::String(string) => string.is_empty(),
            _ => false,
        }
    }

    pub fn is_nullish(&self) -> bool {
        match self {
            Reference::Constant(constant) => constant.is_nullish(),
            _ => false,
        }
    }

    pub(crate) fn op_strict_equal_internal(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::Number(number), Reference::Number(other)) => number.op_strict_equal_internal(other),
            (Reference::Constant(constant), Reference::Constant(other)) => constant.op_strict_equal_internal(other),
            (Reference::String(string), Reference::String(other)) => *string == *other,
            _ => ptr::addr_of!(self) == ptr::addr_of!(other),
        }
    }
}

impl ToString for Reference {
    fn to_string(&self) -> String {
        match self {
            Reference::Object(object) => unimplemented!("object to string"), 
            Reference::Number(number) => number.to_string(),
            Reference::Constant(constant) => constant.to_string(),
            Reference::String(string) => string.to_string(),
            Reference::Array(array) => unimplemented!("array to string"), 
            Reference::Function(function) => unimplemented!("function to string"),
            Reference::Error(error) => error.to_string(),
            Reference::NativeFunction(name, _) => name.to_string(),
        }
    }
}