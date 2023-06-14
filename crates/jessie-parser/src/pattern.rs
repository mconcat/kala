use jessie_ast::*;
use crate::parser;
use crate::{
    VecToken, Token,

    repeated_elements,

    expression,
};

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Patterns, Bindings, Definitions

pub fn binding_pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &param, false).map(|x| Pattern::ArrayPattern(x)),
        Some(Token::LeftBrace) => repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_param, false).map(|x| Pattern::RecordPattern(x)),
        c => state.err_expected("binding pattern", c),
    }
}

// only parses original "pattern" rule from Jessica, not the entire variants of enum Pattern.
// consider changing the name to binding_or_ident_pattern or something...
pub fn pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::LeftBrace) => binding_pattern(state),
        Some(Token::Comma) | Some(Token::RightBracket) => Ok(Pattern::Hole), // Not sure if its the right way...
        _ => // data_literal(state).map(|x| Pattern::DataLiteral(x)).or_else(|_| {
            def_variable(state).map(|x| Pattern::Variable(x))
        //}),
    }
}

pub fn param(state: &mut ParserState) -> Result<Pattern, ParserError> {
    if state.lookahead_1() == Some(Token::DotDotDot) {
        state.consume_1(Token::DotDotDot)?;
        return pattern(state).map(|x| Pattern::Rest(Box::new(x)))
    }

    let pat = pattern(state)?;
    if let Pattern::Variable(ref x) = pat {
        if state.try_proceed(Token::Equal) {
            let expr = expression(state)?;
            return Ok(Pattern::Optional(x.clone(), Box::new(expr)))
        }
    }

    Ok(pat)
}

fn prop_param(state: &mut ParserState) -> Result<PropParam, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| PropParam::Rest(x))
    }

    let key = def_variable(state)?; // def or use XXX

    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            let pat = pattern(state)?;
            Ok(PropParam::KeyValue(key, pat))
        },
        Some(Token::Equal) => {
            state.proceed();
            let expr = expression(state)?;
            Ok(PropParam::Optional(key, expr))
        }
        _ => Ok(PropParam::Shorthand(key)),
    }
}
