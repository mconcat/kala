use crate::{Statement, Expr, Pattern, DataLiteral, Field};
use std::{rc::Rc, cell::OnceCell, cell::RefCell, borrow::BorrowMut, option};
use utils::{OwnedString, SharedString, OwnedSlice};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Option<SharedString>,

    pub captures: OwnedSlice<DeclarationIndex>,

    pub parameters: OwnedSlice<DeclarationIndex>,

    pub declarations: OwnedSlice<Declaration>,

    // block body
    pub statements: OwnedSlice<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Capture {
        name: SharedString,
        variable: VariableCell, // pointing upper function scope
        //index: DeclarationIndex,
    },
    Parameter {
        pattern: Pattern,
        //index: DeclarationIndex,
    },
    OptionalParameter {
        name: SharedString,
        default: Expr,
    },
    Const {
        pattern: Pattern,
        value: Option<Expr>,
       // index: DeclarationIndex,
    },
    Let {
        pattern: Pattern,
        value: Option<Expr>,
        //index: DeclarationIndex,
    },
    Function {
        function: Box<Function>,
        //index: DeclarationIndex,
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationIndex(pub usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    // index of the variable declaration in the innermost function 
    pub declaration_index: DeclarationIndex,

    pub property_access: PropertyAccessChain,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariablePointer(Rc<RefCell<Rc<OnceCell<Variable>>>>);

impl VariablePointer {
    pub fn new() -> Self {
        VariablePointer(Rc::new(RefCell::new(Rc::new(OnceCell::new()))))
    }

    pub fn initialized(declaration_index: DeclarationIndex, property_access: PropertyAccessChain) -> Self {
        let cell = OnceCell::new();
        cell.set(Variable {
            declaration_index,
            property_access,
        }).unwrap();
        VariablePointer(Rc::new(RefCell::new(Rc::new(cell))))
    }

    pub fn set(&mut self, declaration_index: DeclarationIndex, property_access: PropertyAccessChain) -> Result<(), Variable> {
        let inner = (*self.0).borrow_mut();

        inner.set(Variable {
            declaration_index,
            property_access,
        })
    }

    pub fn overwrite(&mut self, other: &Self) {
        let mut inner = (*self.0).borrow_mut();

        *inner = other.0.borrow().clone()
    }

    pub fn new_cell(&self, name: SharedString) -> VariableCell {
        let cell = OnceCell::new();
        VariableCell { name, cell, ptr: self.clone() }
    }

    pub fn is_initialized(&self) -> bool {
        (*self.0.borrow()).get().is_some()
    }

    pub fn is_uninitialized(&self) -> bool {
        (*self.0.borrow()).get().is_none()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableCell {
    pub name: SharedString,
    pub cell: OnceCell<Variable>,
    pub ptr: VariablePointer,
}

impl VariableCell {
    pub fn uninitialized(name: SharedString) -> Self {
        VariableCell {
            name,
            cell: OnceCell::new(),
            ptr: VariablePointer(Rc::new(RefCell::new(Rc::new(OnceCell::new())))),
        }
    }

    pub fn initialized(name: SharedString, declaration_index: DeclarationIndex, property_access: Vec<PropertyAccess>) -> Self {
        let mut cell = OnceCell::new();
        cell.set(Variable {
            declaration_index,
            property_access: PropertyAccessChain::from_vec(property_access),
        }).unwrap();
        let ptr = VariablePointer(Rc::new(RefCell::new(Rc::new(cell.clone()))));
        Self { name, cell, ptr }
    }

    // Must not be called before all the scoping is done
    pub fn get(&self) -> Variable {
        if let Some(var) = self.cell.get() {
            return var.clone()
        }

        let mut inner = (*self.ptr.0).borrow_mut();
        let ptr_var = inner.as_ref().get().unwrap();
        self.cell.set(ptr_var.clone());
        ptr_var.clone()
    }

    pub fn is_uninitialied(&self) -> bool {
        self.cell.get().is_none()
    }

    pub fn merge_into(&mut self, other: &Self) {
        if !self.is_uninitialied() {
            // No need to merge initialized variables
            unreachable!()
        }

        self.ptr.overwrite(&other.ptr)
    }
}

#[derive(Debug, PartialEq, Clone)] 
pub enum PropertyAccess {
    Property(Box<Field>),
    Element(usize),
    // Rest,
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)] 
pub struct PropertyAccessChain(OwnedSlice<PropertyAccess>);

impl PropertyAccessChain {
    pub fn empty() -> Self {
        PropertyAccessChain(OwnedSlice::empty())
    }

    pub fn from_vec(vec: Vec<PropertyAccess>) -> Self {
        PropertyAccessChain(OwnedSlice::from_vec(vec))
    }
/* 
    pub fn push(&mut self, access: PropertyAccess) {
        self.0.push(access)
    }

    pub fn pop(&mut self) -> Option<PropertyAccess> {
        self.0.pop()
    }

    pub fn last_mut(&mut self) -> &mut PropertyAccess {
        self.0.last_mut().unwrap()
    }
*/
}