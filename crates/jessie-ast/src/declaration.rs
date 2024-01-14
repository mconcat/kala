use std::{rc::Rc, fmt::Debug};

use utils::SharedString;

use crate::{Function, Pattern, Expr, OptionalPattern};





impl Debug for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{:?} = {:?}", self.pattern, value),
            None => write!(f, "{:?}", self.pattern),
        }
    }
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