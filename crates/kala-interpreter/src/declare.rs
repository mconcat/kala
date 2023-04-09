use kala_ast::ast;
use kala_ast::pattern::*;
use crate::context::InterpreterContext;
use crate::literal;
use crate::value::JSValue;
use crate::node;
use crate::node::InterpreterF;

pub fn declare_binding_literal(ctx: &mut InterpreterContext, binding: &literal::Literal, value: &JSValue) -> Result<(), String> {
    if value != binding {
        return Err(format!("Cannot assign literal {:?} to {:?}", value, binding))
    }
    Ok(())
}

pub fn declare_binding_object(ctx: &mut InterpreterContext, is_mutable: bool, pat: &ast::pattern::ObjectPattern<InterpreterF>, value: &JSValue) -> Result<(), String> {
    for (i, prop) in pat.properties.iter().enumerate() {
        match prop {
            PropertyPattern::KeyValue(k, v) => {
                let prop = value.get_property(k);
                if prop.is_none() {
                    return Err(format!("Invalid key {:?} in {:?}", k, value))
                }
                declare_binding(ctx, is_mutable,v, &prop)?
            },
            PropertyPattern::Shorthand(k) => {
                let prop = value.get_property(k);
                if prop.is_none() {
                    return Err(format!("Invalid key {:?} in {:?}", k, value))
                }
                if is_mutable {
                    ctx.declare_mutable_binding(k, &prop)?
                } else {
                    ctx.declare_immutable_binding(k, &prop.unwrap() /* TODO: fix unwrap */)?
                }
            },
            PropertyPattern::Rest(pat) => unimplemented!()
        }
    }
    Ok(())
}

pub fn declare_binding_array(ctx: &mut InterpreterContext, is_mutable: bool, pat_arr: &ast::pattern::ArrayPattern<InterpreterF>, value: &JSValue) -> Result<(), String> {
    for (i, pat_elem) in pat_arr.elements.iter().enumerate() {
        match pat_elem {
            ElementPattern::Pattern(pat) => {
                let elem = value.get_index(i.try_into().unwrap());
                if elem.is_none() {
                    return Err(format!("Invalid index {:?} in {:?}", i, value))
                }
                declare_binding(ctx, is_mutable, pat, &elem.map(|x| x.clone()))?
            },
            ElementPattern::Rest(pat) => unimplemented!()
        }
    };
    Ok(())
}

pub fn declare_binding(ctx: &mut InterpreterContext, is_mutable: bool, binding: &ast::Pattern<InterpreterF>, value: &Option<JSValue>) -> Result<(), String> {
    match binding {
        ast::Pattern::Hole => Ok(()),
        ast::Pattern::Identifier(pat) => {
            ctx.declare_mutable_binding(&pat, value)
        },
        ast::Pattern::Literal(pat) => {
            if value.is_none() {
                return Err(format!("Binding assignment should have initial value"))
            }
            declare_binding_literal(ctx, pat, &value.as_ref().unwrap())
        },
        ast::Pattern::Object(pat) => {
            if value.is_none() {
                return Err(format!("Binding assignment should have initial value"))
            }
            declare_binding_object(ctx, is_mutable, pat, &value.as_ref().unwrap())
        },
        ast::Pattern::Array(arr) => {
            if value.is_none() {
                return Err(format!("Binding assignment should have initial value")) 
            }
            declare_binding_array(ctx, is_mutable, arr, &value.as_ref().unwrap()) 
        },
    }
}