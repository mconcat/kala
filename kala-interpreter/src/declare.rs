use kala_ast::ast;
use crate::context::InterpreterContext;
use crate::value::JSValue;

#[inline]
pub fn declare_binding_identifier(ctx: &mut InterpreterContext, kind: ast::DeclarationKind, binding: &ast::Identifier, value: &JSValue) -> bool {
    if kind.is_mutable() {
        ctx.declare_mutable_binding(binding, value)
    } else {
        ctx.declare_immutable_binding(binding, value)
    }

    true
}

pub fn declare_binding_literal(ctx: &mut InterpreterContext, binding: &ast::Literal, value: &JSValue) -> bool {
    match binding {
        ast::Literal::Undefined => if let JSValue::Undefined = value { true } else { false },
        ast::Literal::Null => if let JSValue::Null = value { true },
        ast::Literal::Boolean(v) => if let JSValue::Boolean(v2) = value { v == v2 } else { false },
        ast::Literal::Number(v) => if let JSValue::Number(v2) = value { v == v2 } else { false },
        ast::Literal::String(v) => if let JSValue::String(v2) = value { v == v2 } else { false },
        ast::Literal::Bigint(_) => unimplemented!()
    }
}

pub fn declare_binding_array(ctx: &mut InterpreterContext, kind: ast::DeclarationKind, binding: &ast::pattern::ArrayPattern, value: &JSValue) -> bool {
    unimplemented!();
}

pub fn declare_binding(ctx: &mut InterpreterContext, binding: &ast::Pattern, value: &JSValue) -> bool {
    match binding {
        ast::Pattern::Hole => true,
        ast::Pattern::Identifier(id) => {
            ctx.declare_variable(id, value)
        },
        ast::Pattern::Literal(lit) => {
            declare_binding_literal(ctx, lit, value);
        },
        ast::Pattern::Object(obj) => {
            let obj = value.to_object();
            for (key, value) in obj {
                let key = key.to_string();
                let value = value.to_value();
                let binding = obj
                    .get(&key)
                    .expect("Object key should be present in the object");
                declare_binding(ctx, binding, value);
            }
        },
        ast::Pattern::Array(arr) => {
            let arr = value.to_array();
            for (i, binding) in arr.iter().enumerate() {
                let value = arr
                    .get(i)
                    .expect("Array index should be present in the array");
                declare_binding(ctx, binding, value);
            }
        },
        ast::Pattern::Rest(pat) => unreachable!("rest parameter should appear in object or array pattern");
    }
}