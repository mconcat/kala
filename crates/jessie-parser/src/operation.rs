use jessie_ast::*;
use crate::parser;
use crate::{
    VecToken, Token,

    identifier,

    enclosed_element,
    repeated_elements,

    primary_expr,
    expression,
    arg,
    call_and_unary_op,
};

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////////
// Arithmetic expressions
// From this points, all `_with_leftmost` functions are expected to take the `left` argument already parsed as an CallExpr.

pub fn cond_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("cond_expr_internal");
    if state.try_proceed(Token::Question) {
        let expr1 = expression(state)?;
        state.consume_1(Token::Colon)?;
        let expr2 = expression(state)?;
        Ok(Expr::CondExpr(Box::new(CondExpr(result, expr1, expr2))))
    } else {
        Ok(result)
    }
}

pub fn cond_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    let or_else_expr = or_else_expr_with_leftmost(state, left)?;
    cond_expr_internal(state, or_else_expr)
}

pub fn cond_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    let or_else_expr = or_else_expr_with_leftmost_no_power(state, left)?;
    cond_expr_internal(state, or_else_expr)
}

fn or_else_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("or_else_expr_internal");
    while state.try_proceed(Token::BarBar) {
        let and_then_expr2 = and_then_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Or, result, and_then_expr2)))
    }
    Ok(result)
}

pub fn or_else_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("or_else_expr_with_leftmost");
    let mut result = and_then_expr_with_leftmost(state, left)?;
    or_else_expr_internal(state, result)
}

pub fn or_else_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("or_else_expr_with_leftmost_no_power");
    let mut result = and_then_expr_with_leftmost_no_power(state, left)?;
    or_else_expr_internal(state, result)
}

fn and_then_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("and_then_expr_internal");
    while state.try_proceed(Token::AmpAmp) {
        let eager_expr2 = eager_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::And, result, eager_expr2)))
    }
    Ok(result)
}

pub fn and_then_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("and_then_expr");
    let mut result = eager_expr(state)?;
    and_then_expr_internal(state, result)
}

pub fn and_then_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("and_then_expr_with_leftmost");
    let mut result = eager_expr_with_leftmost(state, left)?;
    and_then_expr_internal(state, result)
}

pub fn and_then_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("and_then_expr_with_leftmost_no_power");
    let mut result = eager_expr_with_leftmost_no_power(state, left)?;
    and_then_expr_internal(state, result)
}

pub fn eager_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("eager_expr_internal");
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::LAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::LessThan, result, shift_expr(state)?)))
            },
            Token::RAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::GreaterThan, result, shift_expr(state)?)))
            },
            Token::LAngleEqual => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::LessThanEqual, result, shift_expr(state)?)))
            },
            Token::RAngleEqual => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::GreaterThanEqual, result, shift_expr(state)?)))
            },
            Token::EqualEqualEqual => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::StrictEqual, result, shift_expr(state)?)))
            },
            Token::BangEqualEqual => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::StrictNotEqual, result, shift_expr(state)?)))
            },
            Token::Ampersand => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitAnd, result, shift_expr(state)?)))
            },
            Token::Bar => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitOr, result, shift_expr(state)?)))
            },
            Token::Caret => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitXor, result, shift_expr(state)?)))
            }, 
            _ => break,
        }
    }

    Ok(result)
}

pub fn eager_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("eager_expr");
    let mut result = shift_expr(state)?;
    eager_expr_internal(state, result)
}

pub fn eager_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("eager_expr_with_leftmost");
    let mut result = shift_expr_with_leftmost(state, left)?;
    eager_expr_internal(state, result)
}

pub fn eager_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("eager_expr_with_leftmost_no_power");
    let mut result = shift_expr_with_leftmost_no_power(state, left)?;
    eager_expr_internal(state, result)
}

fn shift_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("shift_expr_internal");
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::LAngleLAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitLeftShift, result, add_expr(state)?)))
            },
            Token::RAngleRAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitRightShift, result, add_expr(state)?)))
            },
            Token::RAngleRAngleRAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitUnsignedRightShift, result, add_expr(state)?)))
            },
            _ => break,
        }
    }

    Ok(result)
}

pub fn shift_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("shift_expr");
    let mut result = add_expr(state)?;
    shift_expr_internal(state, result)
}

pub fn shift_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("shift_expr_with_leftmost");
    let mut result = add_expr_with_leftmost(state, left)?;
    shift_expr_internal(state, result)
}

pub fn shift_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("shift_expr_with_leftmost_no_power");
    let mut result = add_expr_with_leftmost_no_power(state, left)?;
    shift_expr_internal(state, result)
}

pub fn add_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("add_expr_internal");
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::Plus => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Add, result, mult_expr(state)?)))
            },
            Token::Minus => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Sub, result, mult_expr(state)?)))
            },
            _ => break,
        }
    }

    Ok(result)
}

pub fn add_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("add_expr");
    let mut result = mult_expr(state)?;
    add_expr_internal(state, result)
}

pub fn add_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("add_expr_with_leftmost");
    let mut result = mult_expr_with_leftmost(state, left)?;
    add_expr_internal(state, result)
}

pub fn add_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("add_expr_with_leftmost_no_power");
    let mut result = mult_expr_with_leftmost_no_power(state, left)?;
    add_expr_internal(state, result)
}

pub fn mult_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, ParserError> {
    println!("mult_expr_internal");
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::Asterisk => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mul, result, pow_expr(state)?)))
            },
            Token::Slash => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Div, result, pow_expr(state)?)))
            },
            Token::Percent => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mod, result, pow_expr(state)?)))
            },
           _ => break,
        }
    }

    Ok(result)
}

pub fn mult_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("mult_expr");
    let mut result = pow_expr(state)?;
    mult_expr_internal(state, result)
}

pub fn mult_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("mult_expr_with_leftmost");
    let mut result = pow_expr_with_leftmost(state, left)?;
    mult_expr_internal(state, result)
}

pub fn mult_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("mult_expr_with_leftmost_no_power");
    let mut result = pow_expr_with_leftmost_no_power(state, left)?;
    mult_expr_internal(state, result)
}

fn pow_expr_internal(state: &mut ParserState, result: Expr) -> Result<Expr, ParserError> {
    println!("pow_expr_internal");
    // the case where the leftmost expression is NOT a UnaryExpression. 

    if let Some(la) = state.lookahead_1() {
        if la != Token::AsteriskAsterisk {
            return Ok(result)
        } 
    } else {
        return Ok(result)
    }

    state.proceed(); // consume the power operator

    let right = pow_expr(state)?;
    Ok(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Pow, result, right))))
}

fn pow_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("pow_expr");
    let (left, _) = call_and_unary_op(state)?;
    if let Expr::UnaryExpr(_) = left {
        // Not a UnaryExpression, so we can parse the power expression.
        pow_expr_with_leftmost(state, left)
    } else {
        // Is a UnaryExpression, so we cannot parse the power expression.
        pow_expr_with_leftmost_no_power(state, left)
    }
}

fn pow_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    println!("pow_expr_with_leftmost");
    // the leftmost expression is already parsed as an UpdateExpression, and the power operator can come after.
    pow_expr_internal(state, left)
}

fn pow_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, ParserError> {
    // the leftmost expression is already parsed as a UnaryExpression, and the power operator cannot come after.
    // so we can skip the power expression parsing.
    if let Some(Token::AsteriskAsterisk) = state.lookahead_1() {
        return state.err_expected("no power operator", Some(Token::AsteriskAsterisk))
    } else {
        Ok(left)
    }
}

fn call_post_op(state: &mut ParserState) -> Result<CallPostOp, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftParen) => repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &arg, true).map(|x| CallPostOp::Call(x)),
        Some(Token::LeftBracket) => enclosed_element(state, Token::LeftBracket, Token::RightBracket, &expression).map(|x| CallPostOp::Index(x)),
        Some(Token::Dot) => {
            state.proceed();
            let ident = identifier(state)?;
            Ok(CallPostOp::Member(ident))
        },
        c => state.err_expected("index, member, or call operator", c),
    }
}

pub fn call_expr_internal(state: &mut ParserState, mut expr: Expr) -> Result<(Expr, bool), ParserError> { 
    let mut only_member_post_op = true;

    let mut post_ops = Vec::new();

    while let Ok(post_op) = call_post_op(state) {
        match post_op {
            CallPostOp::Call(_) => { only_member_post_op = false },
            CallPostOp::Index(_) | CallPostOp::Member(_) => {},
        }
        post_ops.push(post_op);
    }

    if !post_ops.is_empty() {
        expr = Expr::CallExpr(Box::new(CallExpr { expr, post_ops, }));
    }

    Ok((expr, only_member_post_op))
}

pub fn call_expr_with_leftmost(state: &mut ParserState, mut expr: Expr) -> Result<Expr, ParserError> {
    // with_leftmost functions take the leftmost expression already parsed as a CallExpression. We can just return it.

    // I believe rust compiler will take care of it
    Ok(expr)
} 

pub fn call_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    let expr = primary_expr(state)?;
    let (result, _) = call_expr_internal(state, expr)?;
    Ok(result)
}



pub fn unary_op(state: &mut ParserState) -> Result<UnaryOp, ParserError> {
    match state.lookahead_1() {
        Some(Token::Plus) => state.proceed_then(UnaryOp::Pos),
        Some(Token::Minus) => state.proceed_then(UnaryOp::Neg),
        Some(Token::Bang) => state.proceed_then(UnaryOp::Not),
        Some(Token::Tilde) => state.proceed_then(UnaryOp::BitNot),
        Some(Token::TypeOf) => state.proceed_then(UnaryOp::TypeOf),
        c => state.err_expected("unary operator", c),
    }
}