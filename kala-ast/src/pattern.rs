////////////////////////////////////////////////////////////////////////
/// Patterns
///
/// Patterns are used in variable declarations, function parameters, and
/// destructuring assignments.

use std::fmt::Debug;

use crate::common::{Identifier, Literal};

use swc_ecma_ast as ast;

pub fn into_identifier<F: PatternF>(expr: ast::Ident) -> F::Identifier{
    let interim: Identifier = expr.into();
    interim.into()
}

pub trait PatternF {
    type Identifier: From<Identifier> + Debug + Clone;
    type Literal: From<Literal> + Debug + Clone;
}

#[derive(Debug, Clone)]
pub enum Pattern<F: PatternF> {
    Identifier(F::Identifier),
    Literal(F::Literal),
    Array(ArrayPattern<F>),
    Object(ObjectPattern<F>),
    Hole,
    // Optional(OptionalPattern),
}

/*
pub struct OptionalPattern {
    pub binding: Identifier,
    pub default: Expression,
}
*/

#[derive(Debug, Clone)]
pub struct ArrayPattern<F: PatternF> {
    pub elements: Vec<ElementPattern<F>>,
}

#[derive(Debug, Clone)]
pub enum ElementPattern<F: PatternF> {
    Pattern(Pattern<F>),
    Rest(Pattern<F>),
}

#[derive(Debug, Clone)]
pub struct ObjectPattern<F: PatternF> {
    pub properties: Vec<PropertyPattern<F>>,
}

#[derive(Debug, Clone)]
pub enum PropertyPattern<F: PatternF> {
    KeyValue(F::Identifier, Pattern<F>),
    Shorthand(F::Identifier),
    // Optional(OptionalPattern),
    Rest(Pattern<F>),
}

impl<F: PatternF> From<ast::Pat> for Pattern<F> {
    fn from(pat: ast::Pat) -> Self {
        match pat {
            ast::Pat::Ident(ident) => Pattern::Identifier(into_identifier::<F>(ident.id)),
            ast::Pat::Array(array) => Pattern::Array(ArrayPattern {
                elements: array.elems.into_iter().map(|e| e.map(|e| e.into()).unwrap_or(ElementPattern::Pattern(Pattern::Hole))).collect(),
            }),
            ast::Pat::Object(object) => Pattern::Object(ObjectPattern {
                properties: object
                    .props
                    .into_iter()
                    .map(|p| match p {
                        ast::ObjectPatProp::KeyValue(key_value) => {
                            match key_value.key {
                                ast::PropName::Ident(ident) => {
                                    PropertyPattern::KeyValue(into_identifier::<F>(ident), (*key_value.value).into())
                                },
                                _ => unimplemented!(),
                            }
                        }
                        ast::ObjectPatProp::Assign(_assign) => {
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

impl<F: PatternF> From<ast::Param> for Pattern<F> {
    fn from(param: ast::Param) -> Self {
        param.pat.into()
    }
}

impl<F: PatternF> From<ast::Pat> for ElementPattern<F> {
    fn from(pat: ast::Pat) -> Self {
        match pat {
            ast::Pat::Rest(rest) => ElementPattern::Rest((*rest.arg).into()),
            _ => ElementPattern::Pattern(pat.into()),
        }   
    }
}