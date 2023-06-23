use jessie_ast::*;
use crate::parser;
use crate::{
    Token, VecToken,
};

use utils::SharedString;
///////////////////////////
// Basic components

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;


pub fn identifier(state: &mut ParserState) -> Result<SharedString, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(s)
        },
        c => state.err_expected("identifier", c),
    }
}

pub fn use_variable(state: &mut ParserState) -> Result<VariableCell, ParserError> {
    let ident = identifier(state)?;
    Ok(state.scope.use_variable(&ident))
}
/* 
pub fn use_variable_with_parsed(state: &mut ParserState, ident: String) -> UseVariable {
    let var = state.scope.use_variable(&ident);
    println!("use variable {:?}", state);
    var 
}

pub fn optional_type_ann(state: &mut ParserState) -> Result<Option<TypeAnn>, ParserError> {
    Ok(None)
}

}*/