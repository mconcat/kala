use utils::{SharedString};

use crate::{Expr, Pattern, Field, OptionalPattern, Block, Variable, Declaration};
use std::{rc::Rc, cell::OnceCell, cell::RefCell, fmt::Debug};

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

#[derive(PartialEq, Clone)]
pub struct LocalVariable {
    pub var: Variable,
    pub is_captured: bool,
}

impl Debug for LocalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LocalVariable { var, is_captured } = self;
        write!(f, "{:?}{}", var, if *is_captured { "!" } else { "" })
    }
}



#[derive(PartialEq, Clone)]
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
    pub locals: Vec<LocalVariable>,

    // hoisted function declarations
    pub functions: Vec<(Variable, Rc<RefCell<Function>>)>,

    // block body
    pub statements: Block, 

    // expression body(arrow function)
    pub expression: Option<Expr>,
}


fn render_variables(variables: &Vec<Variable>) -> String {
    let mut iter = variables.iter();
    let mut result = String::new();
    if let Some(first) = iter.next() {
        result.push_str(&format!("{:?}", first));
        for variable in iter {
            result.push_str(&format!(", {:?}", variable));
        }
    }
    result
}

fn render_functions(functions: &Vec<(Variable, Rc<Function>)>) -> String {
    let mut iter = functions.iter();
    let mut result = String::new();
    if let Some((variable, _)) = iter.next() {
        result.push_str(&format!("{:?}", variable));
        for (variable, _) in iter {
            result.push_str(&format!(", {:?}", variable));
        }
    }
    result
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            FunctionName::Arrow => write!(f, "({}) =>", render_parameters(&self.parameters)),
            FunctionName::Anonymous => write!(f, "function({})", render_parameters(&self.parameters)),
            FunctionName::Named(name) => write!(f, "function {}({:?})", name.0, render_parameters(&self.parameters)),
        }?;

        write!(f, "[{}]", render_variables(&self.captures))?;

        write!(f, "{{{}}}", render_variables(&self.locals))?;

        write!(f, " {:?}", self.statements)
    }
}

impl Function {
    pub fn new(
        name: FunctionName, 
        parameters: Vec<Pattern>, 
        captures: Vec<Variable>,
        locals: Vec<Variable>,
        functions: Vec<(Variable, Rc<RefCell<Function>>)>,
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
