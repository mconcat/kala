use utils::{SharedString};

use crate::{Expr, Pattern, Field, OptionalPattern, Block};
use std::{rc::Rc, cell::OnceCell, cell::RefCell};

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionName {
    Arrow,
    Anonymous,
    Named(SharedString),
}

impl FunctionName {
    pub fn is_named(&self) -> bool {
        match self {
            FunctionName::Named(_) => true,
            _ => false,
        }
    }

    pub fn get_name(&self) -> Option<&SharedString> {
        match self {
            FunctionName::Named(name) => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: FunctionName,

    pub captures: Vec<CaptureDeclaration>, 

    pub parameters: Vec<ParameterDeclaration>, 

    pub locals: Vec<LocalDeclaration>, 

    // block body
    pub statements: Block, 
}

impl Function {
    /* 
    // for testing
    pub fn get_varaible_map(&self) -> FxMap<VariableCell> {
        let mut map = FxMap::new();
        for (i, capture) in self.captures.into_iter().enumerate() {
            match capture {
                CaptureDeclaration::Global { name } => {
                    unimplemented!("global");
                },
                CaptureDeclaration::Local { name, variable } => { 
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Capture(i as u32), vec![])); 
                },
            }
        };

        for (i, parameter) in self.parameters.into_iter().enumerate() {
            match parameter {
                ParameterDeclaration::Optional { name, default } => { 
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Parameter(i as u32), vec![])); 
                },
                ParameterDeclaration::Pattern { pattern } => {
                    pattern.initialize_variable_map(&mut map, DeclarationIndex::Parameter(i as u32), &mut vec![])
                },
                ParameterDeclaration::Variable { name } => {
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Parameter(i as u32), vec![]));
                },
            }
        };

        for (i, local) in self.locals.into_iter().enumerate() {
            match local {
                LocalDeclaration::Const { pattern, value } => {
                    pattern.initialize_variable_map(&mut map, DeclarationIndex::Local(i as u32), &mut vec![])
                },
                LocalDeclaration::Let { pattern, value } => {
                    pattern.initialize_variable_map(&mut map, DeclarationIndex::Local(i as u32), &mut vec![])
                },
                LocalDeclaration::Function { function } => {
                    if let Some(name) = function.name.get_name() {
                        map.insert(name, VariableCell::initialized(name.clone(), DeclarationIndex::Local(i as u32), vec![]));
                    }
                },
            }
        };

        map
    }
    */
}

#[derive(Debug, PartialEq, Clone)]
pub enum CaptureDeclaration {
    Local {
        name: SharedString,
        variable: VariableCell, // pointing upper function scope
    },
    Global {
        name: SharedString,
    },
}

impl CaptureDeclaration {
    pub fn uninitialized(name: SharedString) -> Self {
        CaptureDeclaration::Local {
            name: name.clone(),
            variable: VariableCell::uninitialized(name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterDeclaration {
    Variable {
        name: SharedString,
    },
    Pattern {
        pattern: Pattern,
        //index: DeclarationIndex,
    },
    Optional {
        name: SharedString,
        default: Expr,
    },
}

impl From<Pattern> for ParameterDeclaration {
    fn from(pattern: Pattern) -> Self {
        match pattern {
            Pattern::Variable(var) => ParameterDeclaration::Variable { name: var.name },
            Pattern::Optional(pat) => {
                let OptionalPattern(_, crate::LValueOptional::Variable(var), default) = *pat;
                ParameterDeclaration::Optional { name: var.name, default }
            },
            _ => ParameterDeclaration::Pattern { pattern },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LocalDeclaration { 
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

impl LocalDeclaration {
    pub fn get_initial_value(&self) -> &Option<Expr> {
        match self {
            LocalDeclaration::Const { pattern, value } => value,
            LocalDeclaration::Let { pattern, value } => value,
            LocalDeclaration::Function { function } => unreachable!("function declaration should not be called get_initial_value")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum DeclarationIndex {
    Parameter(u32),
    Capture(u32),
    Local(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    // index of the variable declaration in the innermost function 
    pub declaration_index: DeclarationIndex,

    pub property_access: Vec<PropertyAccess>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariablePointer(Rc<RefCell<Rc<OnceCell<Variable>>>>);

impl VariablePointer {
    pub fn new() -> Self {
        VariablePointer(Rc::new(RefCell::new(Rc::new(OnceCell::new()))))
    }

    pub fn initialized(declaration_index: DeclarationIndex, property_access: &Vec<PropertyAccess>) -> Self {
        let cell = OnceCell::new();
        cell.set(Variable {
            declaration_index,
            property_access: property_access.clone(),
        }).unwrap();
        VariablePointer(Rc::new(RefCell::new(Rc::new(cell))))
    }

    pub fn set(&mut self, declaration_index: DeclarationIndex, property_access: Vec<PropertyAccess>) -> Result<(), Variable> {
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

#[derive(Debug, Clone)]
pub struct VariableCell {
    pub name: SharedString,
    pub cell: OnceCell<Variable>,
    pub ptr: VariablePointer,
}

impl PartialEq for VariableCell {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false
        }

        let self_var = self.get_checked();
        let other_var = other.get_checked();

        self_var == other_var
    }
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
            property_access,
        }).unwrap();
        let ptr = VariablePointer(Rc::new(RefCell::new(Rc::new(cell.clone()))));
        Self { name, cell, ptr }
    }

    // Must not be called before all the scoping is done
    pub fn get(&self) -> Variable {
        println!("Getting variable {:?}", self);

        if let Some(var) = self.cell.get() {
            return var.clone()
        }

        let mut inner = (*self.ptr.0).borrow_mut();
        let ptr_var = inner.as_ref().get().unwrap();
        self.cell.set(ptr_var.clone());
        ptr_var.clone()
    }

    pub fn get_checked(&self) -> Option<Variable> {
        if let Some(var) = self.cell.get() {
            return Some(var.clone())
        }

        let inner = (*self.ptr.0).borrow();
        let ptr_var = inner.as_ref().get()?;
        self.cell.set(ptr_var.clone());
        Some(ptr_var.clone())
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
    Element(u32),
    // Rest,
}