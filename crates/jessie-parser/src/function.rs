use jessie_ast::*;
use crate::{parser, pattern};
use crate::{
    VecToken, Token,

    repeated_elements,

    expression,

    prop_name,
    param,

    block_raw,

    assign_or_cond_or_primary_expression,
};
use utils::{SharedString};
use std::ops::Deref;

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;



pub fn function_expr(state: &mut ParserState) -> Result<Function, ParserError> {
    state.consume_1(Token::Function)?;
    let name = if let Some(Token::Identifier(name)) = state.lookahead_1() {
        state.proceed();
        Some(name)
    } else {
        None
    };

    let function = function_internal(state, name.clone())?;

    // Named function expr should be only locally bound. TODO.
    // For now recursive call is not supported for function expressions
    // state.scope.declare_function(function).ok_or(ParserError::DuplicateDeclaration)?;

    Ok(function)
}

pub fn function_internal(state: &mut ParserState, name: Option<SharedString>) -> Result<Function, ParserError> {

    println!("function_internal");
    let parent_scope = state.enter_function_scope(Vec::new());
    
    let parameter_patterns = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;

    println!("parameter patterns {:?}", parameter_patterns[0]);

    let mut parameters = Vec::with_capacity(parameter_patterns.len());
    state.scope.declare_parameters(parameter_patterns, &mut parameters).ok_or(ParserError::DuplicateDeclaration)?;

    println!("parameters {:?}", parameters);
    println!("scope {:?}", state.scope);

    // TODO: spread parameter can only come at the end

    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            let statements = block_raw(state)?;
            let (declarations, captures) = state.exit_function_scope(parent_scope);
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

/* 
#[derive(Debug, PartialEq, Clone)]
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
*/

fn destructing_array_parameter(state: &mut ParserState) -> Result<CoverParameter, ParserError> {
    let element_patterns = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &pattern, true)?;

    Ok(ArrayPattern(element_patterns))
}
/* 
#[derive(Debug, PartialEq, Clone)]
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
*/
pub fn prop_param(state: &mut ParserState) -> Result<PropParam, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let rest = expression_or_pattern(state)?;
        return Ok(PropDef::Spread(rest.expr));
    }

    let prop_name = prop_name(state)?;
    println!("lookahead {:?}", state.lookahead_1());

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
        Some(Token::QuasiQuote) => {
            unimplemented!("quasiquote")
        },
        _ => {
            state.err_expected(": for property pair", state.lookahead_1())
        }
    }
}

fn destructing_record_parameter_or_record_literal(state: &mut ParserState) -> Result<CoverParameter, ParserError> {

    let cover_props = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &prop_def_or_prop_param
    , true)?;

    let mut props = Vec::with_capacity(cover_props.len());

    let mut is_transmutable_to_param = true;

    for cover_prop in cover_props {
        props.push(cover_prop.prop);
        is_transmutable_to_param &= cover_prop.is_transmutable_to_param;
    }

    Ok(CoverParameter { expr: Expr::Record(Box::new(Record(props))), is_transmutable_to_param })
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
        Some(Token::LeftBracket) => destructing_array_parameter_or_array_literal(state),
        Some(Token::LeftBrace) => destructing_record_parameter_or_record_literal(state),

        // Default pattern or Assignment expressions
        Some(Token::Identifier(_)) => assignment_or_parameter_or_optional(state),

        // Rest parameter
        Some(Token::DotDotDot) => unimplemented!("rest parameter"),
        
        // No other cases are valid for arrow argument
        _ => expression(state).map(CoverParameter::new_expr),
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
        return state.err_expected("=>", state.lookahead_1())
    }
    let parent_scope = state.enter_function_scope(Vec::new());
    let mut parameters = Vec::with_capacity(params.len());
    state.scope.declare_parameters(params, &mut parameters).ok_or(ParserError::DuplicateDeclaration)?;

    let statements = arrow_function_body(state)?;
    let (declarations, captures) = state.exit_function_scope(parent_scope);

    let function = Function {
        name: None,
        captures,
        parameters,
        declarations,
        statements,
    };

    Ok(Expr::ArrowFunc(Box::new(function)))
}