use jessie_ast::*;
use crate::parser;
use crate::{
    VecToken, Token,

    repeated_elements,

    expression,

    param,

    block_raw,

    assign_or_cond_or_primary_expression,
};

type ParserState<'a> = parser::ParserState<'a, VecToken>;
type ParserError = parser::ParserError<Option<Token>>;



pub fn function_expr(state: &mut ParserState) -> Result<Function, ParserError> {
    state.consume_1(Token::Function)?;
    let name = if let Some(Token::Identifier(name)) = state.lookahead_1() {
        state.proceed();
        Some(name)
    } else {
        None
    };

    let function = function_internal(state, name)?;

    state.scope.declare_function(function).ok_or(ParserError::DuplicateDeclaration)?;

    Ok(function)
}

pub fn function_internal(state: &mut ParserState, name: Option<String>) -> Result<Function, ParserError> {
    let mut declarations = Vec::new();
    
    let parent_scope = state.enter_function_scope(&mut declarations);
    
    let parameter_patterns = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;

    let mut parameters = Vec::with_capacity(parameter_patterns.len());
    for param in parameter_patterns {
        let index = state.scope.declare_parameter(param).ok_or(ParserError::DuplicateDeclaration)?;
        parameters.push(index);
    }
 
    // TODO: spread parameter can only come at the end
    println!("function_internal");

    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            let statements = block_raw(state)?;
            let captures = state.exit_function_scope(parent_scope);
            let func = Function {
                declarations,
                name,
                statements,
                captures,
                parameters,
            };
            Ok(func)
        },
        c => state.err_expected(&"a function body", c),
    }
}

fn expression_or_destruction(state: &mut ParserState) -> Result<(Expr, bool), ParserError> {
    // TODO: implement destructing pattern for arrow arguments
    // Array or object pattern cannot come in the lefthand of an assignment,
    // so we can parse a possibly destructing object / array first,
    // and feed it to cond_expr_with_leftmost if an operator follows.
   Ok((expression(state)?, false))
}

fn assignment_or_parameter_or_optional(state: &mut ParserState) -> Result<(Expr, bool), ParserError> {
    let expr = assign_or_cond_or_primary_expression(state)?;

    match expr {
        // Expression can be a parameter if it is a pure variable
        Expr::Variable(_) => Ok((expr, true)),

        // Expression can be a parameter if it is strictly a form of variable = expression
        Expr::Assignment(ref assign) => if let Assignment(AssignOp::Assign, LValue::Variable(_), _) = **assign {
           Ok((expr, true)) 
        } else {
            Ok((expr, false))
        },

        // Otherwise, it is an expression.
        _ => Ok((expr, false)) 
    }
}

// parses a single CPEAAPL argument(not the whole list)
// returns the expression
fn cpeaapl(state: &mut ParserState) -> Result<(Expr, bool), ParserError> {
    match state.lookahead_1() {
        // Destructuring pattern or Expressions
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => expression_or_destruction(state),

        // Default pattern or Assignment expressions
        Some(Token::Identifier(_)) => assignment_or_parameter_or_optional(state),

        // No other cases are valid for arrow argument
        _ => expression(state).map(|x| (x, false)),
    }
}

fn arrow_function_body(state: &mut ParserState) -> Result<Vec<Statement>, ParserError> {
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

// If an expression starts with a left parenthesis, it can be either a parenthesized expression or an arrow function.
// Parenthesized arrow function arguments have the following structure:
// (arg1, arg2, ..., argN)
// Where
// param <- pattern | ...pattern | variable=expression
// pattern <- identifier | [repeated param] | {repeated property}
// property <- identifier | identifier:pattern | ...pattern | identifier=expression
//
// Parenthesized expression cannot have comma inside, but arrow function arguments can.
// Parenthesized expression cannot have spread operator, but arrow function arguments can.
// Parenthesized expression cannot have default value, but arrow function arguments can.
// Parenthesized expression cannot have colons, but arrow function arguments can.
// Function arguments cannot have values except for the righthand side of the default value.
//
pub fn arrow_or_paren_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    // Split into three cases:
    // () => Parenthesized expression cannot be empty, nullary arrow function.
    // (CPEAAPL) => Required to cover both cases only in unary case.
    // (param, ...) => No group expression in Jessie, n-ary arrow function.

    state.proceed(); // consume left paren
    

    // nullary arrow function
    if state.try_proceed(Token::RightParen) {
        state.consume_1(Token::FatArrow)?;

        let mut declarations = vec![];
        let parent_scope = state.enter_function_scope(&mut declarations);

        let statements = arrow_function_body(state)?;
        let captures = state.exit_function_scope(parent_scope);
        // no need to settle, nullary function

        let function = Function {
            name: None,
            captures,
            parameters: vec![],
            declarations,
            statements,
        };

        return Ok(Expr::ArrowFunc(Box::new(function)))
    }

    // unary spread parameter
    if state.try_proceed(Token::DotDotDot) {
        unimplemented!("spread parameter")
        /*
        let arg = param(state)?;

        let mut declarations = vec![];
        let parent_scope = state.enter_function_scope(declarations);
        

        let parameters = Rc::new(Declaration::Parameters(vec![Pattern::Rest(Box::new(arg))]));
        state.scope.settle_declaration(parameters.clone());
        let body = arrow_function_body(state)?;
        let (scope, unbound_uses) = state.exit_block_scope(parent_scope);
        let func = Box::new(Function::from_body(None, parameters, None, body, scope, unbound_uses));
        return Ok(Expr::ArrowFunc(func))
        */
    }

    // try parse a single CPEAAPL first
    match cpeaapl(state)? {
        // Possibly a unary arrow parameter.
        (expr, true) => {
            match state.lookahead_1() {
                // If a comma follows, it is an n-ary arrow function.
                // TODO: spread element should come only at the last 
                Some(Token::Comma) => {
                    state.proceed();
                    let rest = repeated_elements(state, None, Token::RightParen, &|state| param(state), true)?;

                    let mut declarations = vec![];
                    let parent_scope = state.enter_function_scope(&mut declarations);

                    let parameters = Vec::with_capacity(rest.len() + 1);

                    parameters.push(state.scope.declare_parameter(expr.into()).ok_or(ParserError::DuplicateDeclaration)?);
                    for param in rest {
                        parameters.push(state.scope.declare_parameter(param).ok_or(ParserError::DuplicateDeclaration)?);
                    }

                    let statements = arrow_function_body(state)?;
                    let captures = state.exit_function_scope(parent_scope);

                    let function = Function {
                        name: None,
                        captures,
                        parameters,
                        declarations,
                        statements,
                    };

                    return Ok(Expr::ArrowFunc(Box::new(function)))

                },
                // If a right paren follows, try parse a fat arrow.
                Some(Token::RightParen) => {
                    state.proceed();
                    match state.lookahead_1() {
                        // If a fat arrow follows, it is a unary arrow function.
                        Some(Token::FatArrow) => {
                            state.proceed();

                            let mut declarations = vec![];
                            let parent_scope = state.enter_function_scope(&mut declarations);

                            let parameters = vec![state.scope.declare_parameter(expr.into()).ok_or(ParserError::DuplicateDeclaration)?];

                            let statements = arrow_function_body(state)?;

                            let captures = state.exit_function_scope(parent_scope);

                            let function = Function { name: None, captures, parameters, declarations, statements };
                            
                            return Ok(Expr::ArrowFunc(Box::new(function)))
                        },
                        // Otherwise, it is a parenthesized expression.
                        _ => {
                            return Ok(Expr::ParenedExpr(Box::new(expr)))
                        } 
                    }
                }
                // No other token should follow after.
                c => state.err_expected("end of the parenthesized expression or another arrow parameter", c),
            }
        }

        // Not an arrow parameter, should be a parenthesized expression.
        (expr, false) => {
            state.consume_1(Token::RightParen)?;
            return Ok(Expr::ParenedExpr(Box::new(expr)))
        }
    }
}


