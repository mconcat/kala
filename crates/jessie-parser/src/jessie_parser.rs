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

