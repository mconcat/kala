use jessie_ast::*;
use crate::{jessie_parser::{JessieParserState, repeated_elements}, Token, statement::block_raw, expression::{prop_name, expression}, common::use_variable, parser, pattern::{pattern, param}};

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
    let parent_scope = state.enter_function_scope();
    
    let parameter_patterns = repeated_elements
    (state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;

    let mut parameters = Vec::with_capacity(parameter_patterns.len());
    state.scope.declare_parameters(parameter_patterns, &mut parameters).ok_or(ParserError::DuplicateDeclaration)?;

    println!("parameters {:?}", parameters);
    println!("scope {:?}", state.scope);

    // TODO: spread parameter can only come at the end

    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            let statements = Block { statements: block_raw(state)? };
            let declarations = state.exit_function_scope(parent_scope);
            let func = Function {
                name,
                declarations,
                statements,
            };
            Ok(func)
        },
        c => state.err_expected(&"a function body", c),
    }
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
            let var = state.scope.use_variable(&prop_name.dynamic_property);
            Ok(PropParam::Shorthand(prop_name, Box::new(var)))
        },
        Some(Token::QuasiQuote) => {
            unimplemented!("quasiquote")
        },
        la => {
            state.err_expected(": for property pair", la)
        }
    }
}

pub fn arrow_function_body(state: &mut ParserState) -> Result<Vec<Statement>, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            state.proceed();
            block_raw(state)
        },
        _ => {
            let expr = expression(state)?;
            Ok(vec![Statement::Return(Box::new(expr))])
        }
    }
}

pub fn arrow_expr(state: &mut ParserState) -> Result<Expr, ParserError> { 
    let params = repeated_elements(state, Some(Token::ArrowLeftParen), Token::ArrowRightParen, &param, true)?;
    if !state.try_proceed(Token::FatArrow) {
        let la = state.lookahead_1();
        return state.err_expected("=>", la)
    }
    let parent_scope = state.enter_function_scope();
    let mut parameters = Vec::with_capacity(params.len());
    state.scope.declare_parameters(params, &mut parameters).ok_or(ParserError::DuplicateDeclaration)?;

    let statements = Block { statements: arrow_function_body(state)? };
    let declarations = state.exit_function_scope(parent_scope);

    let function = Function {
        name: FunctionName::Arrow,
        declarations,
        statements,
    };

    Ok(Expr::Function(Box::new(function)))
}