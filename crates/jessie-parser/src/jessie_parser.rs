use std::fmt::Debug;
use std::mem::replace;
use std::rc::Rc;

use crate::parser::{self, ParserState}; 
use crate::lexer::{Token};
use jessie_ast::*;
use utils::{MapPool,  Map};

type ParserError = parser::ParserError<Option<Token>>;

#[derive(Debug)]
pub struct JessieParserState {
    pub state: ParserState<Token>,

    pub scope: Vec<Vec<Declaration>>,
}

impl JessieParserState {
    pub fn enter_block(&mut self) {
        self.scope.push(Vec::new());
    }

    pub fn exit_block(&mut self) -> Box<[Declaration]> {
        self.scope.pop().unwrap().into_boxed_slice()
    }

    pub fn new(tokens: Vec<Token>) -> JessieParserState {
        JessieParserState {
            state: ParserState::new(tokens),
            scope: Vec::new(),
        }
    }

    pub fn consume_1(&mut self, token: Token) -> Result<(), ParserError> {
        self.state.consume_1(token)
    }

    pub fn lookahead_1(&mut self) -> Option<Token> {
        self.state.lookahead_1()
    }

    pub fn lookahead_2(&mut self) -> Option<Token> {
        self.state.lookahead_2()
    }

    pub fn proceed(&mut self) {
        self.state.proceed();
    }

    pub fn try_proceed(&mut self, token: Token) -> bool {
        self.state.try_proceed(token)
    }

    pub fn proceed_then<R>(&mut self, r: R) -> Result<R, ParserError> {
        self.state.proceed_then(r)
    }

    pub fn err_expected<T>(&mut self, expected: &'static str, found: Option<Token>) -> Result<T, ParserError> {
        self.state.err_expected(expected, found)
    }
}

pub fn enclosed_element<Data: Debug>(
    state: &mut JessieParserState, 
    open: Token, 
    close: Token, 
    element: &impl Fn(&mut JessieParserState) -> Result<Data, ParserError>
) -> Result<Data, ParserError> {
    state.consume_1(open)?;
    let result = element(state)?;
    state.consume_1(close)?;
    Ok(result)
}

/////////
/// 
/// // comma seperated list of elements, with optional trailing comma
pub fn repeated_elements<Data: Debug>(
    state: &mut JessieParserState,
    open: Option<Token>, 
    close: Token, 
    element: &impl Fn(&mut JessieParserState) -> Result<Data, ParserError>, 
    trailing: bool
) -> Result<Vec<Data>, ParserError> {
    let mut elements = Vec::new();
    if let Some(some_open) = open.clone() {
        state.consume_1(some_open)?;
    }
    loop { // I don't like having loop here
        println!("loop {:?}", elements);
        // consume_whitespace(state);
        if state.lookahead_1() == Some(close.clone()) {
            state.proceed();
            break;
        }
        println!("element start");
        println!("{:?}{:?}", state.lookahead_1(), state.lookahead_2());
        elements.push(element(state)?);
        println!("element end");
        // consume_whitespace(state);
        if state.try_proceed(Token::Comma) {
            if state.lookahead_1() == Some(close.clone()) {
                if trailing {
                    state.proceed();
                    break;
                } else {
                    return state.err_expected("no trailing comma", Some(Token::Comma))
                }
            } 
        } else if state.try_proceed(close.clone()) {
            break
        } else {
            let la = state.lookahead_1();
            return state.err_expected("comma or close", la)
        }
    }

    Ok(elements)
}


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

