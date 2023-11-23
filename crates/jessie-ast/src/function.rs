use utils::{SharedString};

use crate::{Expr, Pattern, Field, OptionalPattern, Block, Variable, Declaration};
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

    // lexical parameters of the function.
    // Negative VariableIndex refers to parameters elements, that does not destructured.
    pub parameters: Vec<Pattern>,

    // lexical captures of the function.
    // the variables are evaluated in the context where the function is defined, when the function is created.
    pub captures: Vec<Variable>,

    // local declarations; consts, lets
    // Positive VariableIndex refers to declaration elements
    pub locals: Vec<Variable>,

    // hoisted function declarations
    pub functions: Vec<(Variable, Rc<Function>)>,

    // block body
    pub statements: Block, 
}

impl Function {
    pub fn new(
        name: FunctionName, 
        parameters: Vec<Pattern>, 
        captures: Vec<Variable>,
        locals: Vec<Variable>,
        functions: Vec<(Variable, Rc<Function>)>,
        statements: Block,
    ) -> Self {
        Self {
            name,
            parameters,
            captures,
            locals,
            functions,
            statements,
        }
    }
}
