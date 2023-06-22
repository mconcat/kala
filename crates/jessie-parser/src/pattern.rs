use jessie_ast::*;
use crate::common::identifier;
use crate::parser;
use crate::{
    VecToken, Token,

    repeated_elements,

    expression,
};

type ParserState<'a> = parser::ParserState<'a, VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Patterns, Bindings, Definitions

pub fn binding_pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &param, false).map(|x| Pattern::ArrayPattern(Box::new(ArrayPattern(x)))),
        Some(Token::LeftBrace) => repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_param, false).map(|x| Pattern::RecordPattern(Box::new(RecordPattern(x)))),
        c => state.err_expected("binding pattern", c),
    }
}

// only parses original "pattern" rule from Jessica, not the entire variants of enum Pattern.
// consider changing the name to binding_or_ident_pattern or something...
pub fn pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::LeftBrace) => binding_pattern(state),
        // Some(Token::Comma) | Some(Token::RightBracket) => Ok(Pattern::Hole), // Not sure if its the right way...
        _ => {// data_literal(state).map(|x| Pattern::DataLiteral(x)).or_else(|_| {
            let name = identifier(state)?;
            let var = state.scope.use_variable(&name);
            Ok(Pattern::Variable(Box::new(var)))
        }
        //}),
    }
}

pub fn param(state: &mut ParserState) -> Result<Pattern, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| Pattern::Rest(Box::new(x)))
    }

    let pat = pattern(state)?;
    if let Pattern::Variable(ref x) = pat {
        if state.try_proceed(Token::Equal) {
            let expr = expression(state)?;
            return Ok(Pattern::optional(**x, expr))
        }
    }

    Ok(pat)
}

fn prop_param(state: &mut ParserState) -> Result<PropParam, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| PropParam::Rest(x))
    }

    let key = identifier(state)?;

    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            let pat = pattern(state)?;
            Ok(PropParam::KeyValue(Box::new(Field::new_dynamic(key)), pat))
        },
        Some(Token::Equal) => {
            unimplemented!("default value in record pattern")
            /* 
            state.proceed();
            let expr = expression(state)?;
            Ok(PropParam::(key, expr))
            */
        }
        _ => {
            let var = state.scope.use_variable(&key);
            let field = Box::new(Field::new_dynamic(key));
            Ok(PropParam::Shorthand(field, var))
        }
    }
}

