use core::{panic};
use std::{mem::{ManuallyDrop, transmute}, rc::{Rc, Weak}, cell::{Cell, RefCell}, any::Any, ops::{Index, IndexMut}, fmt::{Debug, LowerHex}};

use crate::{array::Array, object::{Object, Property}, number::Number, function::{Function, Stack, Frame}, completion::Completion};

use super::{reference::Reference, integer::Integer, constant::Constant};

#[repr(usize)]
#[derive(Debug)]
pub enum SlotTag {
    Reference = 0b00,
    Pointer = 0b10,

    // Integer have 0b01 tag.
    // Having the second lowest bit unset means addition/subtraction
    // does not effect on the actual data.
    Integer = 0b01,
    Constant = 0b11,
}

pub(crate) const MASK: usize = 0b11;

pub union Slot {
    // Raw representation of a slot.
    pub(crate) raw: usize,

    // Reference is a pointer to heap allocated, reference count object.
    // The object will not be deallocated from the heap until all references
    // to it have gone out of scope.
    // Reference is tagged with 00 and not requires detagging before use.
    pub(crate) reference: ManuallyDrop<SlotReference>,

    // Pointer is a pointer to an object that may be heap allocated,
    // but also may be stack or static allocated.
    // Pointer does not guarantee allocation/deallocation safety,
    // and should be treated as a raw pointer. 
    // Static analysis must be done in prior to ensure the safety.
    // Pointer is tagged with 10 and requires detagging before use.
    pointer: ManuallyDrop<SlotPointer>,

    // Integer is a 28-bit/60-bit signed inlined integer.
    // Integer should be heap allocated when captured by a closure.
    // Integer is tagged with 01 and requires detagging before use.
    pub(crate) integer: ManuallyDrop<SlotInteger>,

    // Constant is a inlined constant.
    // Cosntant could be either undefined, null, true, or false.
    // Constant should be heap allocated when captured by a closure.
    // Constant is tagged with 11 and requires detagging before use.
    pub(crate) constant: ManuallyDrop<SlotConstant>,
}

impl Slot {
    pub const TRUE: Self = Self {
        constant: ManuallyDrop::new(SlotConstant(Constant::True)),
    };

    pub const FALSE: Self = Self {
        constant: ManuallyDrop::new(SlotConstant(Constant::False)),
    };

    pub const NULL: Self = Self {
        constant: ManuallyDrop::new(SlotConstant(Constant::Null)),
    };

    pub const UNDEFINED: Self = Self {
        constant: ManuallyDrop::new(SlotConstant(Constant::Undefined)),
    };

    pub const UNINITIALIZED: Self = Self {
        raw: 0,
    };

    pub fn get_tag(&self) -> SlotTag {
        unsafe { transmute(self.raw & MASK) }
    }

    pub fn is_uninitialized(&self) -> bool {
        unsafe{self.raw == 0}
    }
}

impl ToString for Slot {
    fn to_string(&self) -> String {
        if self.is_uninitialized() {
            return "UNINITIALIZED".to_string()
        }

        match self.get_tag() {
            SlotTag::Reference => unsafe { self.reference.0.as_ptr().as_ref().unwrap().to_string() },
            SlotTag::Integer => unsafe { self.integer.0.to_string() },
            SlotTag::Constant => unsafe { self.constant.0.to_string() },
            _ => unreachable!(),
        }
    }
}

impl Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_uninitialized() {
            return write!(f, "UNINITIALIZED")
        }

        unsafe{
            match self.get_tag() {
                SlotTag::Reference => self.unwrap_reference().fmt(f),
                SlotTag::Integer => self.integer.0.fmt(f),
                SlotTag::Constant => self.constant.0.fmt(f),
                SlotTag::Pointer => {
                    write!(f, "*")?;
                    self.unwrap_pointer().fmt(f)
                }
            }
        }
    }
}

impl Clone for Slot {
    fn clone(&self) -> Self {
        if self.is_uninitialized() {
            return Self::UNINITIALIZED
        }

        match self.get_tag() {
            SlotTag::Reference => Self {
                reference: unsafe{ManuallyDrop::new(SlotReference(Rc::clone(&self.reference.0)))},
            },
            SlotTag::Integer => Self {
                integer: unsafe{ManuallyDrop::new(SlotInteger(self.integer.0))},
            },
            SlotTag::Constant => Self {
                constant: unsafe{ManuallyDrop::new(SlotConstant(self.constant.0))},
            },
            SlotTag::Pointer => Self {
                pointer: unsafe{ManuallyDrop::new(SlotPointer(Rc::into_raw(Rc::from_raw(self.pointer.0 as *const Slot).clone())))},
            } 
        }
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        if self.is_uninitialized() {
            return
        }

        match self.get_tag() {
            SlotTag::Reference => unsafe { ManuallyDrop::drop(&mut self.reference) },
            //WEAK_REFERENCE_TAG => unsafe { ManuallyDrop::drop(&mut self.weak_reference) },
            SlotTag::Integer => unsafe { ManuallyDrop::drop(&mut self.integer) },
            SlotTag::Constant => unsafe { ManuallyDrop::drop(&mut self.constant) },
            SlotTag::Pointer => unsafe { ManuallyDrop::drop(&mut ManuallyDrop::new(Rc::from_raw(self.pointer.0))) },
        }
    }
}

impl Slot {
    pub fn new_variable_slot() -> Self {
        Self {
            pointer: ManuallyDrop::new(SlotPointer::new(Slot::UNINITIALIZED)),
        }
    }

    pub fn new_null() -> Self {
        Self {
            constant: ManuallyDrop::new(SlotConstant(Constant::Null)),
        }
    }

    pub fn new_undefined() -> Self {
        Self {
            constant: ManuallyDrop::new(SlotConstant(Constant::Undefined)),
        }
    }

    pub fn new_false() -> Self {
        Self {
            constant: ManuallyDrop::new(SlotConstant(Constant::False)),
        }
    }

    pub fn new_true() -> Self {
        Self {
            constant: ManuallyDrop::new(SlotConstant(Constant::True)),
        }
    }

    pub fn new_integer(integer: i64) -> Self {
        if let Some(integer) = Integer::new(integer) {
            return Self {
                integer: ManuallyDrop::new(SlotInteger(integer)),
            }
        }

        Self::new_number(integer, 0)
    }

    pub fn new_string(string: impl Into<Rc<str>>) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::String(string.into()))))),
        }
    }

    pub fn new_array(array: Vec<Slot>) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::Array(Array(array)))))),
        }
    }

    pub fn new_object(properties: Vec<Property>) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::Object(Object{properties}))))),
        }
    }

    pub fn new_boolean(boolean: bool) -> Self {
        if boolean {
            Self::new_true()
        } else {
            Self::new_false()
        }
    }

    pub fn new_number(i: i64, f: u64) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::Number(Number::new(i, f)))))),
        }
    }

    pub fn new_native_function(name: impl Into<Rc<str>>, function: Rc<RefCell<dyn FnMut(&mut [Slot]) -> Completion>>) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::NativeFunction(name.into(), function))))),
        }
    }

    pub const fn new_uninitialized() -> Self {
        Self::UNINITIALIZED
    }

    pub fn new_function(
        name: Option<Rc<str>>,
        //parameters_len: usize,
        //captures: Vec<Slot>,
        //locals_len: usize,
        function: Rc<dyn Fn(&mut Frame, Vec<Slot>) -> Completion>,
    ) -> Self {
        Self {
            reference: ManuallyDrop::new(SlotReference(Rc::new(Cell::new(Reference::Function(Function {
                name,
                //captures,
                function,
            }))))),
        }
    }

    pub fn unwrap_reference(&self) -> &Reference {
        unsafe { &*self.reference.0.as_ptr() }
    }

    pub fn unwrap_mut_reference(&mut self) -> &mut Reference {
        unsafe { &mut *self.reference.0.as_ptr() }
    }
/* 
    pub fn unwrap_weak_reference(&self) -> &Reference {
        unsafe { &(*self.weak_reference.0.into_raw()).into_inner() }
    }
*/
    pub fn unwrap_integer(&self) -> Integer {
        unsafe { self.integer.0 }
    }

    pub fn unwrap_mut_integer(&mut self) -> &mut Integer {
        unsafe { &mut self.integer.0 }
    }

    pub fn unwrap_constant(&self) -> Constant {
        unsafe { self.constant.0 }
    }

    pub fn unwrap_mut_constant(&mut self) -> &mut Constant {
        unsafe { &mut self.constant.0 }
    }

    pub fn unwrap_pointer(&self) -> &Slot {
        unsafe { &*((self.pointer.0 as usize & !MASK) as *const Slot) }
    }

    pub fn unwrap_mut_pointer(&mut self) -> &mut Slot {
        unsafe { &mut *((self.pointer.0 as usize & !MASK) as *mut Slot)  }
    }

    pub fn call(&self, frame: &mut Frame, arguments: &mut Vec<Slot>) -> Completion {
        if self.is_uninitialized() {
            panic!("uninitialized slot")
        }

        match self.get_tag() {
            SlotTag::Reference => match self.unwrap_reference() {
                Reference::Function(function) => (function.function)(frame, arguments.clone()),
                Reference::NativeFunction(_, function) => function.borrow_mut()(&mut arguments[..]),
                _ => Completion::Throw(Slot::new_string("TypeError: not a function")),
            },
            _ => Completion::Throw(Slot::new_string("TypeError: not a function")),
        }
    }

    pub fn is_nullish(&self) -> bool {
        if self.is_uninitialized() {
            panic!("uninitialized slot")
        }

        match self.get_tag() {
            SlotTag::Pointer => self.unwrap_pointer().is_nullish(),
            SlotTag::Constant => unsafe { self.constant.0.is_nullish() },
            SlotTag::Reference => unsafe { match self.unwrap_reference() {
                Reference::Constant(constant) => constant.is_nullish(),
                Reference::Object(object) => object.is_empty(),
                _ => false,
            } },
            SlotTag::Integer => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        if self.is_uninitialized() {
            panic!("uninitialized slot")
        }

        match self.get_tag() {
            SlotTag::Constant => unsafe { self.constant.0.is_falsy() },
            SlotTag::Reference => !self.unwrap_reference().is_falsy(),
            SlotTag::Integer => self.unwrap_integer() != Integer(0),
            _ => true,
        }
    }

    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }
}

impl Slot {
    pub fn get_property_by_string(&mut self, index: Rc<str>) -> Option<&Property> {
        if self.is_uninitialized() {
            panic!("uninitialized slot")
        }

        match self.get_tag() {
            SlotTag::Reference => match self.unwrap_mut_reference() {
                Reference::Object(object) => object.index_property_by_string(index),
                Reference::Array(array) => unimplemented!("wrapped array object"),
                Reference::Constant(constant) => unimplemented!("wrapped constant object"),
                Reference::Number(number) => unimplemented!("wrapped number object"),
                Reference::String(string) => unimplemented!("wrapped string object"),
                Reference::Function(function) => unimplemented!("wrapped function object"),
                Reference::Error(error) => unimplemented!("wrapped error object"),
                Reference::NativeFunction(name, _) => unimplemented!("wrapped native function object")
            }
            SlotTag::Integer => unimplemented!("wrapped number object"),
            SlotTag::Constant => unimplemented!("wrapped constant object"),
            _ => unreachable!("invalid slot tag")
        }
    }

    pub fn mut_property_by_string(&mut self, index: Rc<str>) -> Option<&mut Property> {
        if self.is_uninitialized() {
            panic!("uninitialized slot")
        }

        match self.get_tag() {
            SlotTag::Reference => match self.unwrap_mut_reference() {
                Reference::Object(object) => object.index_mut_property_by_string(index),
                Reference::Array(array) => unimplemented!("wrapped array object"),
                Reference::Constant(constant) => unimplemented!("wrapped constant object"),
                Reference::Number(number) => unimplemented!("wrapped number object"),
                Reference::String(string) => unimplemented!("wrapped string object"),
                Reference::Function(function) => unimplemented!("wrapped function object"),
                Reference::Error(error) => unimplemented!("wrapped error object"),
                Reference::NativeFunction(name, _) => unimplemented!("wrapped native function object")
            }
            SlotTag::Integer => unimplemented!("wrapped number object"),
            SlotTag::Constant => unimplemented!("wrapped constant object"),
            _ => unreachable!("invalid slot tag")
        }
    }

    pub fn set(&mut self, slot: Slot) {
        match self.get_tag() {
            SlotTag::Pointer => *self.unwrap_mut_pointer() = slot,
            _ => *self = slot,
        }
    }
}

// SlotReference holds shared reference to a heap allocated object.
// We use Rc<Cell<Object>> instead of Rc<RefCell<Object>> because
// borrow semantics safety is not a constraint in ECMAScript, and
// Cell provides better performance than RefCell.
#[repr(transparent)]

#[derive(Clone)]
pub struct SlotReference(pub Rc<Cell<Reference>>);

impl SlotReference {
    pub(crate) fn new(reference: Reference) -> Self {
        Self(Rc::new(Cell::new(reference)))
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SlotPointer(pub *const Slot);

impl SlotPointer {
    pub(crate) fn new(slot: Slot) -> Self {
        let rc = Rc::into_raw(Rc::new(Cell::new(slot)));

        // Cell<T> and T have the same memory layout.
        let ptr = (rc as usize | SlotTag::Pointer as usize) as *const Slot;

        Self(ptr)
    }

}

// WeakReference is used when the lifetime of the variable is guaranteed to be
// within the lifetime of any of the owner(s) of the variable.
// Usually, the owner of the variable is the function that created the variable,
// but the ownership could escape the function when there is a reference to the 
// variabe with a longer lifetime.
// pub struct SlotWeakReference(pub Weak<Cell<Reference>>);


// Integer is for small, inlined signed integer.
// shifted by 4 bits for tagging, 28-bit in 32-bit system, 60-bit in 64-bit system.
#[repr(transparent)]
#[derive(Clone)]
pub struct SlotInteger(pub Integer); 

#[repr(transparent)]
#[derive(Clone)]
pub struct SlotConstant(pub Constant);
/* 
impl LowerHex for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_uninitialized() {
            return write!(f, "UNINITIALIZED")
        }

        unsafe{
            match self.get_tag() {
                SlotTag::Reference => self.reference.0.as_ptr().fmt(f),
                SlotTag::Integer => self.integer.0.fmt(f),
                SlotTag::Constant => self.constant.0.fmt(f),
                _ => unreachable!(),
            }
        }
    }
}
*/