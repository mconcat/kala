use kala_ast::ast;
use kala_ast::pattern::*;
use crate::context::InterpreterContext;
use crate::literal;
use crate::value::JSValue;
use crate::lexical;
use crate::lexical::InterpreterF;

#[inline]
pub fn declare_binding_identifier(ctx: &mut InterpreterContext, kind: &ast::DeclarationKind, binding: &lexical::Identifier, value: &JSValue) -> bool {
    if kind.is_mutable() {
        ctx.declare_mutable_binding(binding, value);
    } else {
        ctx.declare_immutable_binding(binding, value);
    }

    true
}

pub fn declare_binding_literal(ctx: &mut InterpreterContext, binding: &literal::Literal, value: &JSValue) -> bool {
    value == binding
}

pub fn declare_binding_array(ctx: &mut InterpreterContext, kind: &ast::DeclarationKind, binding: &ArrayPattern<InterpreterF>, value: &JSValue) -> bool {
    unimplemented!();
}

pub fn declare_binding(ctx: &mut InterpreterContext, kind: &ast::DeclarationKind, binding: &ast::Pattern<InterpreterF>, value: &JSValue) -> bool {
    match binding {
        ast::Pattern::Hole => true,
        ast::Pattern::Identifier(pat) => {
            if kind.is_mutable() {
                ctx.declare_mutable_binding(&pat, value)
            } else {
                ctx.declare_immutable_binding(&pat, value)
            }
        },
        ast::Pattern::Literal(pat) => {
            declare_binding_literal(ctx, pat, value)
        },
        ast::Pattern::Object(pat) => {
            for (i, prop) in pat.properties.iter().enumerate() {
                match prop {
                    PropertyPattern::KeyValue(k, v) => {
                        let value = value.get_property(k);
                        if value.is_none() {
                            return false
                        }
                        if !declare_binding(ctx, &kind, v, &value.unwrap()) {
                            return false
                        }
                    },
                    PropertyPattern::Shorthand(k) => {
                        let value = value.get_property(k);
                        if value.is_none() {
                            return false
                        }
                        if !declare_binding_identifier(ctx, &kind, k, &value.unwrap()) {
                            return false
                        }
                    },
                    PropertyPattern::Rest(pat) => unimplemented!()
                }
            }
            true
        },
        ast::Pattern::Array(arr) => {
            for (i, elem) in arr.elements.iter().enumerate() {
                match elem {
                    ElementPattern::Pattern(pat) => {
                        let value = value.get_index(i.try_into().unwrap());
                        if value.is_none() {
                            return false
                        }
                        if !declare_binding(ctx, kind, pat, value.unwrap()) {
                            return false
                        }
                    },
                    ElementPattern::Rest(pat) => unimplemented!()
                }
            };
            true 
        },
    }
}