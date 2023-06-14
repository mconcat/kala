use std::rc::Rc;

use crate::parser::{self}; 
use crate::lexer::{Token, VecToken, repeated_elements, enclosed_element};
use jessie_ast::*;

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

// stuffs to care about:
// https://github.com/mozilla-spidermonkey/jsparagus/blob/master/js-quirks.md#readme

/*
pub fn module_binding(state: &mut ParserState, proxy: MutableDeclarationPointer) -> Result<DeclarationPointer, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state, proxy)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?;
            let decl = Rc::new(Declaration::Const(pattern, expr));
            Ok(decl)
        },
        _ => {
            let ident = def_variable(state, proxy)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?; // TODO: check if right
            Ok(ModuleBinding::VariableBinding(ident, Some(expr)))
        }
    }
}
*/

///////////////////////////
// Basic components

pub fn identifier(state: &mut ParserState) -> Result<String, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(s)
        },
        c => state.err_expected("identifier", c),
    }
}

pub fn def_variable(state: &mut ParserState, proxy: &MutableDeclarationPointer) -> Result<DefVariable, ParserError> {
    let ident = identifier(state)?;
    let var = state.scope.def_variable(proxy, &ident).or_else(|msg| state.err_scope(&msg, ident))?;
    // let type_ann = optional_type_ann(state)?;
    Ok(var)
}

pub fn use_variable(state: &mut ParserState) -> Result<UseVariable, ParserError> {
    let ident = identifier(state)?;
    let var = state.scope.use_variable(&ident);
    println!("use variable {:?}", state);
    Ok(var)
}

pub fn use_variable_with_parsed(state: &mut ParserState, ident: String) -> UseVariable {
    let var = state.scope.use_variable(&ident);
    println!("use variable {:?}", state);
    var 
}

pub fn optional_type_ann(state: &mut ParserState) -> Result<Option<TypeAnn>, ParserError> {
    Ok(None)
}

pub fn prop_name(state: &mut ParserState) -> Result<PropName, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(PropName::Ident(s))
        },
        Some(Token::String(s)) => {
            state.proceed();
            Ok(PropName::String(s))
        },
        Some(Token::Integer(s)) => {
            state.proceed();
            Ok(PropName::Number(s))
        },
        c => state.err_expected("property name", c),
    }
}