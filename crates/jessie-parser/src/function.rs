use std::rc::Rc;

use jessie_ast::*;
use crate::{jessie_parser::{JessieParserState, repeated_elements}, Token, statement::{block_raw, self}, expression::{prop_name, expression}, common::use_variable, parser, pattern::{pattern, param}};

type ParserState = JessieParserState;
type ParserError = parser::ParserError<Option<Token>>;



pub fn function_expr(state: &mut ParserState) -> Result<Function, ParserError> {
    state.consume_1(Token::Function)?;
    let name = if let Some(Token::Identifier(name)) = state.lookahead_1() {
        state.proceed();
        FunctionName::Named(name)
    } else {
        FunctionName::Anonymous
    };

    let function = function_internal(state, name)?;

    // Named function expr should be only locally bound. TODO.
    // For now recursive call is not supported for function expressions
    // state.scope.declare_function(function).ok_or(ParserError::DuplicateDeclaration)?;

    Ok(function)
}

pub fn function_internal(state: &mut ParserState, name: FunctionName) -> Result<Function, ParserError> {

    println!("function_internal");
    state.enter_block();

    let parameters = repeated_elements
    (state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?.into_boxed_slice();

    println!("parameters {:?}", parameters);

    // TODO: spread parameter can only come at the end

    let statements = block_raw(state)?;
    let declarations = state.exit_block();
    let statements = Block { declarations, statements };
    let func = Function {
        name: name,
        parameters,
        body: ExprOrBlock::Block(statements),
        scope: None,
    };
    Ok(func)
}

pub fn prop_param(state: &mut ParserState) -> Result<PropParam, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        return Ok(PropParam::Rest(Box::new(use_variable(state)?)));
    }

    let prop_name = prop_name(state)?;
    println!("lookahead {:?}", state.lookahead_1());

    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            Ok(PropParam::KeyValue(prop_name, pattern(state)?))
        },
        Some(Token::LeftParen) => {
            unimplemented!("method def")
            /* 
            let method_def = method_def(state)?;
            Ok(PropDef::MethodDef(method_def))
            */
        },
        Some(Token::Comma) | Some(Token::RightBrace) => {
            Ok(PropParam::Shorthand(prop_name.clone(), Box::new(Variable::new(prop_name.name))))
        },
        Some(Token::QuasiQuote) => {
            unimplemented!("quasiquote")
        },
        la => {
            state.err_expected(": for property pair", la)
        }
    }
}

pub fn arrow_function_body(state: &mut ParserState) -> Result<Box<[Statement]>, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            block_raw(state)
        },
        _ => {
            let expr = expression(state)?;
            Ok(Box::new([Statement::Return(Box::new(expr))]))
        }
    }
}

pub fn arrow_expr(state: &mut ParserState) -> Result<Expr, ParserError> { 
    let parameters = repeated_elements(state, Some(Token::ArrowLeftParen), Token::ArrowRightParen, &param, true)?.into_boxed_slice();
    if !state.try_proceed(Token::FatArrow) {
        let la = state.lookahead_1();
        return state.err_expected("=>", la)
    }

    state.enter_block();
    let statements = arrow_function_body(state)?;
    let declarations = state.exit_block();

    let body = Block { statements, declarations };

    let function = Function {
        name: FunctionName::Arrow,
        parameters,
        body: ExprOrBlock::Block(body),
        scope: None,
    };

    Ok(Expr::Function(Box::new(function)))
}