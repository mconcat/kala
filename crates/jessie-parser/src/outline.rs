// Basic type checker that does not require complex type inference.
// The main purpose of this module is to assign fixed offset to the object fields. We don't really care about the precise types.

use crate::{
    parser,

    Token, VecToken,
};  

type ParserState<'a> = parser::ParserState<'a, VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

pub enum Outline {
    Primitive,

    // Complex types
    Array,
    Tuple(usize),
    Record(Box<Vec<String>>),
    Function(usize, bool),
}

pub fn index(state: &mut ParserState) -> Result<usize, ParserError> {
    state.consume_1(Token::LeftBracket)?;
    let index = state.consume_1(Token::Number)?;
    state.consume_1(Token::RightBracket)?;
    Ok(index)
}

pub fn member(state: &mut ParserState) -> Result<String, ParserError> {
    state.consume_1(Token::Dot)?;
    let member = state.consume_1(Token::Identifier)?;
    Ok(member)
}

pub fn call(state: &mut ParserState) -> Result<(), ParserError> {
    state.consume_1(Token::LeftParen)?;
    repeated_elements(state, Some(Token::Comma), Token::RightParen, &expr, true/*Check it*/)?;
    Ok(())
}