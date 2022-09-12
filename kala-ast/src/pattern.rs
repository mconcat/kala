////////////////////////////////////////////////////////////////////////
/// Patterns
///
/// Patterns are used in variable declarations, function parameters, and
/// destructuring assignments.

use crate::common::{Identifier, Literal};

use swc_ecma_ast as ast;

pub enum Pattern {
    Identifier(Identifier),
    Literal(Literal),
    Array(ArrayPattern),
    Object(ObjectPattern),
    Hole,
    Rest(Box<Pattern>),
    // Optional(OptionalPattern),
}

/*
pub struct OptionalPattern {
    pub binding: Identifier,
    pub default: Expression,
}
*/

pub struct ArrayPattern {
    pub elements: Vec<Pattern>,
}

pub struct ObjectPattern {
    pub properties: Vec<PropertyPattern>,
}

pub enum PropertyPattern {
    KeyValue(Identifier, Pattern),
    Shorthand(Identifier),
    // Optional(OptionalPattern),
    Rest(Pattern),
}

impl From<ast::Pat> for Pattern {
    fn from(pat: ast::Pat) -> Self {
        match pat {
            ast::Pat::Ident(ident) => Pattern::Identifier(Identifier::from(ident.id)),
            ast::Pat::Array(array) => Pattern::Array(ArrayPattern {
                elements: array.elems.into_iter().map(|e| e.map(|e| e.into()).unwrap_or(Pattern::Hole)).collect(),
            }),
            ast::Pat::Object(object) => Pattern::Object(ObjectPattern {
                properties: object
                    .props
                    .into_iter()
                    .map(|p| match p {
                        ast::ObjectPatProp::KeyValue(key_value) => {
                            PropertyPattern::KeyValue(
                                Identifier::from(key_value.key),
                                (*key_value.value).into(),
                            )
                        }
                        ast::ObjectPatProp::Assign(assign) => {
                            /*
                            PropertyPattern::Optional(OptionalPattern {
                                binding: Identifier::from(assign.key),
                                default: (*assign.value.unwrap()).into(),
                            })
                            */
                            unimplemented!("Optional pattern is not implemented");
                        }
                        ast::ObjectPatProp::Rest(rest) => {
                            PropertyPattern::Rest((*rest.arg).into())
                        }
                    })
                    .collect(),
            }),
            _ => unimplemented!(),   
        }
    }
}

impl From<ast::Param> for Pattern {
    fn from(param: ast::Param) -> Self {
        param.pat.into()
    }
}
