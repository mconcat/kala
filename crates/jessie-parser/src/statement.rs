use std::rc::Rc;

use jessie_ast::*;
use crate::{jessie_parser::{JessieParserState, repeated_elements}, parser, Token, pattern::binding_pattern, expression, common::{use_variable, identifier}, function::function_internal};

type ParserState = JessieParserState; 
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Statements

// statementItem
pub fn statement(state: &mut ParserState) -> Result<Statement, ParserError> {
    // putting whitespace in consumes is a hack, need to fix later
    match state.lookahead_1() {
        Some(Token::LeftBrace) => block(state).map(|x| Statement::Block(Box::new(x))), // TODO: implement statement level record literal?
        Some(Token::Const) => const_decl(state).map(|x| Statement::VariableDeclaration(Box::new(x))),
        Some(Token::Let) => let_decl(state).map(|x| Statement::VariableDeclaration(Box::new(x))),
        Some(Token::Function) => function_decl(state).map(|(index, decl)| Statement::FunctionDeclaration(index, decl)),
        Some(Token::If) => if_statement(state).map(|x| Statement::IfStatement(Box::new(x))),
        Some(Token::While) => while_statement(state).map(|x| Statement::WhileStatement(Box::new(x))),
        Some(Token::Try) => {
            unimplemented!("try statement")
            //state.proceed();
            //try_statement(state).map(Statement::TryStatement)
        },
        Some(Token::Throw) => {
            state.proceed();
            let res = expression(state).map(|x| Statement::Throw(Box::new(x)));
            state.consume_1(Token::Semicolon)?;
            res
        },
        Some(Token::Continue) => {
            state.proceed();
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::Continue)
        },
        Some(Token::Break) => {
            state.proceed();
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::Break)
        },
        Some(Token::Return) => {
            state.proceed();
            if state.try_proceed(Token::Semicolon) {
                Ok(Statement::ReturnEmpty)
            } else {
                let e = expression(state)?;
                state.consume_1(Token::Semicolon)?;
                Ok(Statement::Return(Box::new(e)))
            }
        },
        _ => {
            let e = expression(state)?;
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::ExprStatement(Box::new(e)))
        }
    }
}

fn const_decl(state: &mut ParserState) -> Result<Vec<(u32, Rc<LocalDeclaration>)>, ParserError> {
    state.consume_1(Token::Const)?;
    repeated_elements(state, None, Token::Semicolon, &|state| {
        let (pattern, init) = binding(state)?;
        println!("const_decl {:?} {:?}", pattern, init);
        state.scope.declare_const(pattern, init.unwrap()).ok_or(ParserError::DuplicateDeclaration)
    }, false)
}

fn let_decl(state: &mut ParserState) -> Result<Vec<(u32, Rc<LocalDeclaration>)>, ParserError> {
    state.consume_1(Token::Let)?;
    repeated_elements(state, None, Token::Semicolon, &|state| {
        let (pattern, init) = binding(state)?;
        state.scope.declare_let(&pattern, init).ok_or(ParserError::DuplicateDeclaration)
    }, false)
}

fn function_decl(state: &mut ParserState) -> Result<(u32, Rc<LocalDeclaration>), ParserError> {
    state.consume_1(Token::Function)?;
    let name = identifier(state)?;
    let parent_scope = state.enter_block_scope();
    // TODO: support recursive reference to function
    let function = function_internal(state, FunctionName::Named(name))?;
    state.exit_block_scope(parent_scope);
    state.scope.declare_function(function).ok_or(ParserError::DuplicateDeclaration)
}

pub fn binding(state: &mut ParserState) -> Result<(Pattern, Option<Expr>), ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = expression(state)?;
            Ok((pattern, Some(expr)))
        },
        _ => {
            let var = use_variable(state)?;
            let expr = if state.try_proceed(Token::Equal) {
                Some(expression(state)?)
            } else {
                None
            };
            Ok((Pattern::Variable(Box::new(var)), expr))
        }
    }
}

pub fn block(state: &mut ParserState) -> Result<Block, ParserError> {
    let parent_scope = state.enter_block_scope();

    let statements = block_raw(state)?;

    // Unbound uses list is only needed for function declarations, so we can ignore it here.
    state.exit_block_scope(parent_scope);

    Ok(Block{statements})
}

pub fn block_raw(state: &mut ParserState) -> Result<Vec<Statement>, ParserError> {
    state.consume_1(Token::LeftBrace)?;

    let mut statements = vec![];
    while state.lookahead_1() != Some(Token::RightBrace) {
        statements.push(statement(state)?);
    }

    state.consume_1(Token::RightBrace)?;

    Ok(statements)
}

fn if_statement(state: &mut ParserState) -> Result<IfStatement, ParserError> {
    state.consume_1(Token::If)?;
    state.consume_1(Token::LeftParen)?;
    let condition = expression(state)?;
    state.consume_1(Token::RightParen)?;
    let consequent = block(state)?;

    let alternate = if state.try_proceed(Token::Else) {
        if state.try_proceed(Token::If) {
            ElseArm::ElseIf(if_statement(state).map(Box::new)?)
        } else {
            ElseArm::Else(block(state)?)
        }
    } else {
        ElseArm::NoElse
    };

    Ok(IfStatement { condition, consequent, alternate })
}

pub fn while_statement(state: &mut ParserState) -> Result<WhileStatement, ParserError> {
    state.consume_1(Token::While)?;
    state.consume_1(Token::LeftParen)?;
    let condition = expression(state)?;
    state.consume_1(Token::RightParen)?;
    let body = block(state)?;

    Ok(WhileStatement { condition, body })
}


