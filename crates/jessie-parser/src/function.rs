use jessie_ast::*;
use crate::parser;
use crate::{
    VecToken, Token,

    repeated_elements,

    expression,

    prop_name,
    param,

    block_raw,

    assign_or_cond_or_primary_expression,
};
use utils::{SharedString, OwnedSlice};
use std::ops::Deref;

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

pub fn function_internal(state: &mut ParserState, name: Option<SharedString>) -> Result<Function, ParserError> {
    let mut declarations = Vec::new();
    
    let parent_scope = state.enter_function_scope(&mut declarations);
    
    let parameter_patterns = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;

    let mut parameters = Vec::with_capacity(parameter_patterns.len());
    for param in parameter_patterns.into_iter() {
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
                declarations: OwnedSlice::from_vec(declarations),
                name,
                statements,
                captures,
                parameters: OwnedSlice::from_vec(parameters),
            };
            Ok(func)
        },
        c => state.err_expected(&"a function body", c),
    }
}

struct CoverParameter {
    expr: Expr,
    is_transmutable_to_param: bool,
}

impl Deref for CoverParameter {
    type Target = Expr;
    fn deref(&self) -> &Expr {
        &self.expr
    }
}

impl CoverParameter {
    fn new(expr: Expr) -> Self {
        CoverParameter {
            expr,
            is_transmutable_to_param: true,
        }
    }

    fn new_expr(expr: Expr) -> Self {
        CoverParameter { 
            expr,
            is_transmutable_to_param: false,
        }
    }

    fn assert_not_transmutable_to_param(&mut self) {
        self.is_transmutable_to_param = false;
    }

    fn into_expr(self) -> Expr {
        self.expr
    }

    fn into_param(self) -> Pattern {
        if self.is_transmutable_to_param {
            unsafe {std::mem::transmute(self.expr)}
        } else {
            panic!("CoverParameter is not transmutable to param");
        }
    }
}

fn destructing_array_parameter_or_array_literal(state: &mut ParserState) -> Result<CoverParameter, ParserError> {
    let mut is_transmutable_to_param = true;

    let elements = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &|state| {
        let elem = expression_or_pattern(state)?;
        is_transmutable_to_param &= elem.is_transmutable_to_param;
        Ok(elem.expr)
    }, true)?;

    Ok(CoverParameter { expr: Expr::Array(Box::new(Array(elements))), is_transmutable_to_param })
}
pub struct CoverProperty {
    prop: PropDef,
    is_transmutable_to_param: bool,
}

impl Deref for CoverProperty {
    type Target = PropDef;
    fn deref(&self) -> &PropDef {
        &self.prop
    }
}

impl CoverProperty {
    fn new(prop: PropDef) -> Self {
        CoverProperty {
            prop,
            is_transmutable_to_param: true,
        }
    }

    fn new_prop(prop: PropDef) -> Self {
        CoverProperty { 
            prop,
            is_transmutable_to_param: false,
        }
    }

    fn assert_not_transmutable_to_param(&mut self) {
        self.is_transmutable_to_param = false;
    }

    pub fn into_prop(self) -> PropDef {
        self.prop
    }

    fn into_param(self) -> PropParam {
        if self.is_transmutable_to_param {
            unsafe {std::mem::transmute(self.prop)}
        } else {
            panic!("CoverProperty is not transmutable to param");
        }
    }
}

pub fn prop_def_or_prop_param(state: &mut ParserState) -> Result<CoverProperty, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let rest = expression_or_pattern(state)?;
        return Ok(CoverProperty{
            prop: PropDef::Spread(rest.expr),
            is_transmutable_to_param: rest.is_transmutable_to_param,
        })
    }
    
    if state.lookahead_1() == Some(Token::QuasiQuote) {
        return state.err_unimplemented("quasiquote")
    }

    let prop_name = prop_name(state)?;
    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            let expr = expression_or_pattern(state)?;
            Ok(CoverProperty{
                prop: PropDef::KeyValue(prop_name, expr.expr),
                is_transmutable_to_param: expr.is_transmutable_to_param,
            })
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
            Ok(CoverProperty{
                prop: PropDef::Shorthand(prop_name, var),
                is_transmutable_to_param: true,
            })
        },
        _ => {
            state.err_expected(":", state.lookahead_1())
        }
    }
}

fn destructing_record_parameter_or_record_literal(state: &mut ParserState) -> Result<CoverParameter, ParserError> {
    let mut is_transmutable_to_param = true;

    let elements = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &|state| {
        let elem = prop_def_or_prop_param(state)?;
        is_transmutable_to_param &= elem.is_transmutable_to_param;
        Ok(elem.prop)
    }, true)?;

    Ok(CoverParameter { expr: Expr::Record(Box::new(Record(elements))), is_transmutable_to_param })
}

fn assignment_or_parameter_or_optional(state: &mut ParserState) -> Result<CoverParameter, ParserError> {
    let expr = assign_or_cond_or_primary_expression(state)?;

    match expr {
        // Expression can be a parameter if it is a pure variable
        Expr::Variable(_) => Ok(CoverParameter::new(expr)),

        // Expression can be a parameter if it is strictly a form of variable = expression
        Expr::Assignment(ref assign) => if let Assignment(AssignOp::Assign, LValue::Variable(_), _) = **assign {
           Ok(CoverParameter::new(expr)) 
        } else {
            Ok(CoverParameter::new_expr(expr))
        },

        // Otherwise, it is an expression.
        _ => Ok(CoverParameter::new_expr(expr)) 
    }
}

// parses a single CPEAAPL argument(not the whole list)
// returns the expression
// https://ui.toast.com/posts/ko_20221116_4
fn expression_or_pattern(state: &mut ParserState) -> Result<CoverParameter, ParserError> {
    match state.lookahead_1() {
        // Destructuring pattern or Expressions
        Some(Token::LeftBrace) => destructing_array_parameter_or_array_literal(state),
        Some(Token::LeftBracket) => destructing_record_parameter_or_record_literal(state),

        // Default pattern or Assignment expressions
        Some(Token::Identifier(_)) => assignment_or_parameter_or_optional(state),

        // Rest parameter
        Some(Token::DotDotDot) => unimplemented!("rest parameter"),
        
        // No other cases are valid for arrow argument
        _ => expression(state).map(CoverParameter::new_expr),
    }
}

fn arrow_function_body(state: &mut ParserState) -> Result<OwnedSlice<Statement>, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            state.proceed();
            block_raw(state)
        },
        _ => {
            let expr = expression(state)?;
            Ok(OwnedSlice::from_slice(&[Statement::Return(Box::new(expr))]))
        }
    }
}


////////
/// 
/// CPEAAPL :
/// ( Expression ) // Both valid for parenthesized and arrow function
/// ( Expression , ) // INVALID as group expressions are not allowed
/// ( ) // Valid for arrow function
/// ( ... BindingIdentifier ) // Valid for arrow function
/// ( ... BindingPattern ) // Valid for arrow function
/// ( Expression , ... BindingIdentifier ) 
/// ( Expression , ... BindingPattern )
/// 

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
            parameters: OwnedSlice::empty(),
            declarations: OwnedSlice::from_vec(declarations),
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

    let expr = expression_or_pattern(state)?;

    if !expr.is_transmutable_to_param {
        state.consume_1(Token::RightParen)?;
        return Ok(Expr::ParenedExpr(Box::new(expr.expr)))
    }

    // Possibly a unary arrow parameter.
    match state.lookahead_1() {
        // If a comma follows, it is an n-ary arrow function.
        // TODO: spread element should come only at the last
        Some(Token::Comma) => {
            state.proceed();
            let rest = repeated_elements(state, None, Token::RightParen, &|state| param(state), true)?;

            let mut declarations = vec![];
            let parent_scope = state.enter_function_scope(&mut declarations);

            let parameters = Vec::with_capacity(rest.len() + 1);

            parameters.push(state.scope.declare_parameter(expr.into_param()).ok_or(ParserError::DuplicateDeclaration)?);
            for param in rest {
                parameters.push(state.scope.declare_parameter(param).ok_or(ParserError::DuplicateDeclaration)?);
            }

            let statements = arrow_function_body(state)?;
            let captures = state.exit_function_scope(parent_scope);

            let function = Function {
                name: None,
                captures,
                parameters: OwnedSlice::from_vec(parameters),
                declarations: OwnedSlice::from_vec(declarations),
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

                    let parameters = OwnedSlice::from_slice(&[state.scope.declare_parameter(expr.into_param()).ok_or(ParserError::DuplicateDeclaration)?]);

                    let statements = arrow_function_body(state)?;

                    let captures = state.exit_function_scope(parent_scope);

                    let function = Function { name: None, captures, parameters, declarations: OwnedSlice::from_vec(declarations), statements };
                    
                    return Ok(Expr::ArrowFunc(Box::new(function)))
                },
                // Otherwise, it is a parenthesized expression.
                _ => {
                    return Ok(Expr::ParenedExpr(Box::new(expr.into_expr())))
                } 
            }
        }
        // No other token should follow after.
        c => state.err_expected("end of the parenthesized expression or another arrow parameter", c),
    }
}


