use std::rc::Rc;

use utils::SharedString;

use crate::{Function, Pattern, Expr, OptionalPattern};

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Const(Rc<Vec<VariableDeclaration>>),
    Let(Rc<Vec<VariableDeclaration>>),
    Function(Rc<Function>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub pattern: Pattern,
    pub value: Option<Expr>,
}
/* 
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
            Pattern::Variable(var) => ParameterDeclaration::Variable { name: var.get_name().clone() },
            Pattern::Optional(pat) => {
                let OptionalPattern(_, crate::LValueOptional::Variable(var), default) = *pat;
                ParameterDeclaration::Optional { name: var.get_name().clone(), default }
            },
            _ => ParameterDeclaration::Pattern { pattern },
        }
    }
}*/