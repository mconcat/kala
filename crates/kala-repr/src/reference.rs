use std::{any::Any, ops::{Index, IndexMut}, ptr, cell::Ref, mem::ManuallyDrop};

use utils::SharedString;

use crate::{number::NUMBER_ZERO, object::Property, slot::Slot};

use super::{number::Number, object::Object, constant::Constant, array::Array, function::{StaticFunction}};

pub enum Reference {
    Object(Object),
    Number(Number),
    Constant(Constant),
    String(SharedString),
    Array(Array),

    StaticFunction(StaticFunction<Box<dyn Any>>), // no capture, just function pointer
    //Closure(Closure), // closure with captured environment variable
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
            Reference::StaticFunction(function) => unimplemented!("static function to string"),
        }
    }
}