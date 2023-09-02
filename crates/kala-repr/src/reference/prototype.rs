use std::mem::transmute;

use utils::SharedString;

use crate::array::ArrayPrototype;
use crate::memory::alloc::{Ref};
use crate::slot::{Slot, SlotTag};

use crate::function::{FunctionPrototype, Variable};

#[derive(Clone)]
pub enum Prototype {
    Object(Ref<ObjectPrototype>),
    Array(Ref<ArrayPrototype>),
    TypedArray(Ref<TypedArrayPrototype>),
    Function(Ref<FunctionPrototype<()>>),
    Number(Ref<NumberPrototype>),
    String(Ref<StringPrototype>),
    Boolean(Ref<BooleanPrototype>),
    Error(Ref<ErrorPrototype>),
}

impl Prototype {
    pub fn object() -> Self {
        Self::Object(Ref::new(ObjectPrototype(), SlotTag::Reference))
    }

    pub fn array(elements: Vec<Slot>) -> Self {
        Self::Array(Ref::new(ArrayPrototype{elements}, SlotTag::Reference))
    }

    pub fn function<Code>(name: SharedString, code: Code, captures: Vec<Variable>) -> Self {
        let ptr = 
        Ref::new(FunctionPrototype::new(name, code, captures), SlotTag::Reference);

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
