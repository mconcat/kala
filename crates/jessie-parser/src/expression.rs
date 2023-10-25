use jessie_ast::*;
use crate::{jessie_parser::{JessieParserState, repeated_elements}, function::{function_expr, arrow_expr}, Token, operation::{cond_expr_with_leftmost, cond_expr_with_leftmost_no_power, unary_op, call_expr_internal}, common::use_variable, parser};

type ParserState = JessieParserState; 
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Expressions

// Top level expression parser. Same with AssignExpr.
// assignExpr =
//   | ArrowFunc
//   | ParenedExpr
//   | FunctionExpr
//   | CondExpr // = CondExpr | BinaryExpr | UnaryExpr | CallExpr
//   | DataStructure
//   | QuasiExpr
//   | UseVariable
//   | LValue PostOp
//   | LValue AssignOp AssignExpr
//
//
// When parsing an expression, we track the following two cover possiblity:
// LValue or CondExpr, for the valid lefthand operand for assignment operators.
// ArrowFunction argument or Parenthesized Expression, for the valid lefthand for fat arrow. 
pub fn expression(state: &mut ParserState) -> Result<Expr, ParserError> {
    match state.lookahead_1() {
        // Function expression
        // function f(x) { return x + 1; }
        
        Some(Token::Function) => function_expr(state).map(|x| Expr::Function(Box::new(x))),

        // Parenthesized expression or arrow function
        // (x + y)
        // (x, y) => x + y
        Some(Token::ArrowLeftParen) => arrow_expr(state),
        Some(Token::LeftParen) => {
            state.proceed();
            let expr = expression(state)?;
            state.consume_1(Token::RightParen)?;
            // if the expression is parenthesized expression(a primary expression), 
            // it can be a leftmost expression for a CondExpr.
            let (post_op_expr, _) = call_and_unary_op_internal(state, Expr::ParenedExpr(Box::new(expr)), vec![])?;
            cond_expr_with_leftmost(state, post_op_expr)
        },

        _ => assign_or_cond_or_primary_expression(state),
    }
}



// The result of call_and_unary_op can be one of
// - primary_expr()s: paren, function, literal, array, record, variable
// - call_expr wrapping them(if exists)
// - unary_expr wrapping them(if exists)
// any other cases are unreachable!.
pub fn call_and_unary_op(state: &mut ParserState) -> Result<(Expr, bool), ParserError> {
    // 1. UnaryOp fast path
    let mut preops = vec![];
    while let Ok(preop) = unary_op(state) {
        preops.push(preop)
    }

    // 2. Leftmost PrimaryExpression
    let expr = primary_expr(state)?;

    call_and_unary_op_internal(state, expr, preops)
}

fn call_and_unary_op_internal(state: &mut ParserState, expr: Expr, preops: Vec<UnaryOp>) -> Result<(Expr, bool), ParserError> {

    // 2-1. MemberPostOp fast path
    // If the leftmost node has CallPostOps(but without Call), it could be either an AssignExpr or a CondExpr(CallExpr).
    let (call_expr, only_member_post_op) = call_expr_internal(state, expr)?;

    // apply unaryops(lower precedence than call ops)
    if !preops.is_empty() {
        let unary_expr = Expr::UnaryExpr(Box::new(UnaryExpr { op: preops, expr: call_expr }));
        Ok((unary_expr, only_member_post_op))
    } else {
        Ok((call_expr, only_member_post_op))
    }

}



pub fn assign_or_cond_or_primary_expression(state: &mut ParserState) -> Result<Expr, ParserError> {
    // Leftmost node of the assignment expression could be either PrimaryExpression or Variable.
    // Leftmost node of the conditional expression could be either UnaryOp(in case of UpdateExpr) or PrimaryExpression(in case of CallExpr).

    // We take the following approach:
    // 1. Try parsing a UnaryOp. If it succeeds, we know that we are parsing an CondExpr(with an UnaryExpr as the leftmost node). Power operator should not be parsed in this case. Return UnaryExpr parsing result.
    // 2. Try parsing a PrimaryExpression. If it succeeds, we know that we are parsing either an AssignExpr or a CondExpr(with a PrimaryExpression as the leftmost node).
    // 2-1. Try parsing a MemberPostOp. If it succeeds, we know that we are parsing either an AssignExpr or a CondExpr(with a CallExpr as the leftmost node). If it fails, check if the PrimaryExpression is a variable. If it is not, we are parsing a CondExpr. Return CondExpr parsing result. Otherwise, proceed to 3.
    // 2-2. UNIMPLEMENTED: If the PrimaryExpression is a variable, check if a fat arrow follows. If it does, we are parsing an ArrowFunc with a single parameter. Return ArrowFunc parsing result.
    // 3. Try parsing an AssignOp. If it succeeds, we know that we are parsing an AssignExpr. Coerce the primary expression to a LValue and return the AssignExpr parsing result. If it fails, we are parsing a CondExpr.
    // 4. Try parsing an ternary/binary operator. If it succeeds, we know that we are parsing a CondExpr. Return CondExpr parsing result.
    // 5. If we reach here, we are parsing a PrimaryExpression. Return the parsing result.
    // TODO: support quasi expression

    println!("assign_or_cond_or_something");

    let (expr, only_member_post_op) = call_and_unary_op(state)?;

    println!("expr {:?} {:?}", expr, only_member_post_op);

    match expr {
        Expr::DataLiteral(_) | Expr::Function(_) => {
            // not a lvalue, cannot be an assignment.
            // jump to cond expression parsing.
            return cond_expr_with_leftmost(state, expr)
        }
        Expr::UnaryExpr(_) => {
            // preops exist, cannot be an assignment.
            // jump to cond expression parsing.
            // no power operators can come after.
            return cond_expr_with_leftmost_no_power(state, expr);
        },
        Expr::CallExpr(_) => {
            // if function call exists in the postops, cannot be an assignment.
            // jump to cond expression parsing.
            if !only_member_post_op {
                return cond_expr_with_leftmost(state, expr)
            }
            // otherwise, continue
        },
        Expr::Variable(_) => {
            /* 
            if state.try_proceed(Token::FatArrow) {
                let statements = arrow_function_body(state)?;
                return Ok(Expr::ArrowFunc(Box::new(Function { params: vec![expr], statements })))
            }
            */
            // variable could be either an assignment or a condexpr.
            // continue
        },
        Expr::Array(_) | Expr::Record(_) => {
            // Jessie specification does not allow destructuring pattern appearing as a LValue outside of declarations. Continue to cond expression parsing.
            return cond_expr_with_leftmost(state, expr)
        }
        Expr::ParenedExpr(_) => {
            unreachable!("parenthesized expression should not appear at the left side of an assignment. arrow_or_paren_expr takes priority. not a Jessie spec")
        },
        _ => unreachable!("call_and_unary_op should not return other types of expression"),
    }

    // At this point, expression is either a variable or a call expression(with only member post ops) with primary expression as its leftmost.

    // 3. AssignExpr parsing
    if let Some(op) = assign_op(state) {
        // Assignment operator exists and the expression is coercible into LValue. 
        let lvalue = expr.into(); // must work
        let right = expression(state)?;
        return Ok(Expr::Assignment(Box::new(Assignment ( op, lvalue, right ))));
    }

    // 4. CondExpr parsing
    if lookahead_operator(state) {
        return cond_expr_with_leftmost(state, expr);
    }
    
    // 5. PrimaryExpression or CallExpr
    Ok(expr)
}

fn assign_op(state: &mut ParserState) -> Option<AssignOp> {
    match state.lookahead_1() {
        Some(Token::Equal) => {
            state.proceed();
            Some(AssignOp::Assign)
        },
        Some(Token::PlusEqual) => {
            state.proceed();
            Some(AssignOp::AssignAdd)
        },
        Some(Token::MinusEqual) => {
            state.proceed();
            Some(AssignOp::AssignSub)
        },
        Some(Token::AsteriskEqual) => {
            state.proceed();
            Some(AssignOp::AssignMul)
        },
        Some(Token::SlashEqual) => {
            state.proceed();
            Some(AssignOp::AssignDiv)
        },
        Some(Token::PercentEqual) => {
            state.proceed();
            Some(AssignOp::AssignMod)
        },
        /*
        Some(Token::CaretEqual) => {
            state.proceed();
            Some(AssignOp::AssignExp)
        },
        Some(Token::AmpersandEqual) => {
            state.proceed();
            Some(AssignOp::AssignAnd)
        },
        Some(Token::PipeEqual) => {
            state.proceed();
            Some(AssignOp::AssignOr)
        },
        Some(Token::LeftShiftEqual) => {
            state.proceed();
            Some(AssignOp::)
        },
        Some(Token::RightShiftEqual) => {
            state.proceed();
            Some(AssignOp::RightShiftEqual)
        },
        Some(Token::UnsignedRightShiftEqual) => {
            state.proceed();
            Some(AssignOp::UnsignedRightShiftEqual)
        },
        */
        _ => None,
    }
}

pub fn primary_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    println!("primary_expression {:?}", state);
    match state.lookahead_1() {
        Some(Token::LeftParen) => {
            state.proceed();
            let e = expression(state)?;
            state.consume_1(Token::RightParen)?;
            Ok(Expr::ParenedExpr(Box::new(e)))
        },
        Some(Token::QuasiQuote) => unimplemented!("QuasiExpr not implemented"),
        Some(Token::LeftBracket) => array(state).map(|x| Expr::Array(Box::new(x))),
        Some(Token::LeftBrace) => record(state).map(|x| Expr::Record(Box::new(x))),
        Some(Token::String(s)) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::String(s.into())))),
        Some(Token::Integer(n)) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::Integer(n)))),
        Some(Token::Decimal(i, f)) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::Decimal(i, f)))),
        Some(Token::Null) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::Null))),
        Some(Token::True) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::True))),
        Some(Token::False) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::False))),
        Some(Token::Undefined) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::Undefined))),
        Some(Token::Bigint(sign, abs)) => state.proceed_then(Expr::DataLiteral(Box::new(DataLiteral::Bigint(sign, abs)))),
        Some(Token::Function) => function_expr(state).map(|x| Expr::Function(Box::new(x))),
        _ => use_variable(state).map(|x| Expr::Variable(Box::new(x))),
    }
}
pub fn array(state: &mut ParserState) -> Result<Array, ParserError> {
    let elements = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &mut element, true)?;
    Ok(Array(elements))
}

pub fn element(state: &mut ParserState) -> Result<Expr, ParserError> {
    let expr = if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Expr::Spread(Box::new(expr))
    } else {
        expression(state)?
    };

    Ok(expr)
}

pub fn prop_def(state: &mut ParserState) -> Result<PropDef, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        return Ok(PropDef::Spread(expr))
    }

    let prop_name = prop_name(state)?;
    match state.lookahead_1() {
        // Method
        Some(Token::LeftParen) => {
            unimplemented!("method def")
        },
        // KeyValue
        Some(Token::Colon) => {
            state.proceed();
            let expr = expression(state)?;
            Ok(PropDef::KeyValue(prop_name, expr))
        },
        // Shorthand
        Some(Token::Comma) | Some(Token::RightBrace) => {
            let var = state.scope.use_variable(&prop_name.dynamic_property);
            Ok(PropDef::Shorthand(prop_name, Box::new(var)))
        },
        la => {
            state.err_expected(": for property pair", la)
        }
    }
}

pub fn record(state: &mut ParserState) -> Result<Record, ParserError> {
    let props = repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &mut prop_def, true)?;
    Ok(Record(props))
}
/* 
pub fn pure_prop_def(state: &mut ParserState) -> Result<PropDef, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Ok(PropDef::Spread(expr))
    } else {
        let prop_name = prop_name(state)?;
        if state.lookahead_1() == Some(Token::LeftParen) {
            unimplemented!()
            /* 
            let method_def = method_def(state)?;
            Ok(PurePropDef::MethodDef(method_def))
            */
        } else {
            match prop_name {
                PropName::Ident(ident) => {

                },
                _ => err
            }
            use_variable_with_parsed(state, prop_name.)
            Ok(PropDef::Shorthand(prop_name))
        }
    }
}
*/
/*
pub fn prop_name(state: &mut ParserState) -> Result<PropName, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(PropName::Ident(s))
        },
        Some(Token::String(s)) => {
            state.proceed();
            Ok(PropName::String(s))
        },
        Some(Token::Integer(s)) => {
            state.proceed();
            Ok(PropName::Number(s))
        },
        c => state.err_expected("property name", c),
    }
}
*/

pub fn prop_name(state: &mut ParserState) -> Result<Box<Field>, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(Box::new(Field::new_dynamic(s)))
        },
        /* 
        Some(Token::String(s)) => {
            state.proceed();
            Ok(Box::new(Field::String(s)))
        },
        Some(Token::Integer(s)) => {
            state.proceed();
            Ok(Box::new(Field::Number(s)))
        },
        */
        c => state.err_expected("property name", c),
    }
}



pub fn arg(state: &mut ParserState) -> Result<Expr, ParserError> {
    let is_spread = state.try_proceed(Token::DotDotDot);

    let expr = expression(state)?;

    if is_spread {
        match state.lookahead_1() {
            Some(Token::Comma) => {
                match state.lookahead_2() {
                    Some(Token::RightParen) => {
                        // spread is the last argument, ok
                    },
                    la => return state.err_expected("right paren", la),
                }
            }
            Some(Token::RightParen) => {
                // spread is the last argument, ok
            }
            la => return state.err_expected("comma or right paren", la),
        }

        return Ok(Expr::Spread(Box::new(expr)))
    }

    Ok(expr)
}



fn lookahead_operator(state: &mut ParserState) -> bool {
    match state.lookahead_1() {
        Some(Token::Question)
        | Some(Token::Plus)
        | Some(Token::Minus)
        | Some(Token::Asterisk)
        | Some(Token::Slash)
        | Some(Token::Percent)
        | Some(Token::AsteriskAsterisk)
        | Some(Token::Ampersand)
        | Some(Token::Bar)
        | Some(Token::Caret)
        | Some(Token::LAngle)
        | Some(Token::RAngle)
        | Some(Token::LAngleEqual)
        | Some(Token::RAngleEqual)
        | Some(Token::EqualEqualEqual)
        | Some(Token::BangEqualEqual)
        | Some(Token::LAngleLAngle)
        | Some(Token::RAngleRAngle)
        | Some(Token::RAngleRAngleRAngle)
        | Some(Token::AmpAmp)
        | Some(Token::BarBar)
        | Some(Token::QuestionQuestion) => true,
        _ => false,
    }
}