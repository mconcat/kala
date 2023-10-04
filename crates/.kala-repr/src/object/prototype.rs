use std::mem::transmute;

use utils::SharedString;

use crate::array::ArrayPrototype;
use crate::slot::{Slot, SlotTag};

use crate::function::{FunctionPrototype};

/* 
#[derive(Clone)]
pub enum Prototype {
    Any(*mut Object),
    Object(ObjectPrototype),
    Array(Box<ArrayPrototype>),
    TypedArray(TypedArrayPrototype),
    Function(Box<FunctionPrototype<()>>),
    Number(NumberPrototype),
    String(Box<StringPrototype>),
    Boolean(BooleanPrototype),
    Error(Box<ErrorPrototype>),
}
*/
/* 
impl Prototype {
    pub fn object() -> Self {
        Self::Object(ObjectPrototype())
    }

    pub fn array(elements: Vec<Slot>) -> Self {
        Self::Array(Box::new(ArrayPrototype{elements}))
    }

    pub fn let_array(elements: Vec<Slot>) -> Self {
        Self::Array(Box::new(ArrayPrototype{elements}))
    }

    pub fn function<Code>(name: Option<SharedString>, code: Code, captures: Vec<Slot>) -> Self {
        let ptr = 
        Box::new(FunctionPrototype::new(name, code, captures));

        Self::Function(unsafe{transmute(ptr)}) // ereasing type information
    }

    pub fn get_function<Code>(&self) -> Option<&FunctionPrototype<Code>> {
        if let Self::Function(ptr) = self {
            return Some(unsafe{transmute(ptr)})
        }

        None
    }
}

#[derive(Clone)]
pub struct ObjectPrototype ();


#[derive(Clone)]
pub struct TypedArrayPrototype (/*TODO */);


#[derive(Clone)]
pub struct NumberPrototype(pub u64);

#[derive(Clone)]
pub struct StringPrototype(pub String);

#[derive(Clone)]
pub struct BooleanPrototype(pub bool);

#[derive(Clone)]
pub struct ErrorPrototype(pub String);
*/