use crate::parser::{self, ArrayLike};
use crate::lexer::{self, Token, VecToken, repeated_elements, enclosed_element};
use crate::jessie_types::*;
use crate::jessie_operation::*;

type ParserState = parser::ParserState<VecToken>;

///////////////////////
// Module

pub fn module_body(state: &mut ParserState) -> Result<ModuleBody, String> {
    let mut items = Vec::new();

    while let Some(_) = state.lookahead_1() {
        items.push(module_item(state)?);
    }

    Ok(ModuleBody(items))
}

pub fn module_item(state: &mut ParserState) -> Result<ModuleItem, String> {
    module_decl(state).map(ModuleItem::ModuleDecl) // TODO
}

pub fn module_decl(state: &mut ParserState) -> Result<ModuleDecl, String> {
    state.consume_1(Token::Const)?;
    repeated_elements(state, None, Token::Semicolon, &module_binding, false).map(ModuleDecl)
}

pub fn hardened_expr(state: &mut ParserState) -> Result<HardenedExpr, String> {
    Ok(HardenedExpr(expression(state)?))
}

pub fn module_binding(state: &mut ParserState) -> Result<ModuleBinding, String> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?;
            Ok(ModuleBinding::PatternBinding(pattern, expr))
        },
        _ => {
            let ident = def_variable(state)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?; // TODO: check if right
            Ok(ModuleBinding::VariableBinding(ident, Some(expr)))
        }
    }
}

///////////////////////
// Patterns, Bindings, Definitions

pub fn binding_pattern(state: &mut ParserState) -> Result<Pattern, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &param, false).map(|x| Pattern::ArrayPattern(x, None/*TODO */)),
        Some(Token::LeftBrace) => repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_param, false).map(|x| Pattern::RecordPattern(x, None/*TODO */)),
        _ => Err("Expected binding pattern".to_string()),
    }
}

// only parses original "pattern" rule from Jessica, not the entire variants of enum Pattern.
// consider changing the name to binding_or_ident_pattern or something...
pub fn pattern(state: &mut ParserState) -> Result<Pattern, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::LeftBrace) => binding_pattern(state),
        Some(Token::Comma) | Some(Token::RightBracket) => Ok(Pattern::Hole), // Not sure if its the right way...
        _ => // data_literal(state).map(|x| Pattern::DataLiteral(x)).or_else(|_| {
            identifier(state).map(|x| Pattern::Variable(x, None/*TODO */))
        //}),
    }
}

pub fn param(state: &mut ParserState) -> Result<Pattern, String> {
    if state.lookahead_1() == Some(Token::DotDotDot) {
        state.consume_1(Token::DotDotDot)?;
        return pattern(state).map(|x| Pattern::Rest(Box::new(x), None/*TODO */))
    }

    let pat = pattern(state)?;
    if let Pattern::Variable(ref x, ref ann) = pat {
        if ann.is_some() {
            unimplemented!("Type annotations on parameters are not supported yet")
        }
        
        if state.try_proceed(Token::Equal) {
            let expr = expression(state)?;
            return Ok(Pattern::Optional(x.clone(), Box::new(expr), optional_type_ann(state)?))
        }
    }

    Ok(pat)
}

pub fn prop_param(state: &mut ParserState) -> Result<PropParam, String> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| PropParam::Rest(x))
    }

    let key = identifier(state)?;

    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            let pat = pattern(state)?;
            Ok(PropParam::KeyValue(key, pat))
        },
        Some(Token::Equal) => {
            state.proceed();
            let expr = expression(state)?;
            Ok(PropParam::Optional(key, expr))
        }
        _ => Ok(PropParam::Shorthand(key)),
    }
}

///////////////////////
// Statements

// statementItem
pub fn statement(state: &mut ParserState) -> Result<Statement, String> {
    // putting whitespace in consumes is a hack, need to fix later
    match state.lookahead_1() {
        Some(Token::LeftBrace) => block(state).map(Statement::Block), // TODO: implement statement level record literal?
        Some(Token::Const) => {
            state.proceed();
            declaration(state, DeclarationKind::Const).map(Statement::Declaration)
        },
        Some(Token::Let) => {
            state.proceed();
            declaration(state, DeclarationKind::Let).map(Statement::Declaration)
        },
        Some(Token::Function) => function_decl(state).map(Statement::FunctionDeclaration),
        Some(Token::If) => if_statement(state).map(Statement::IfStatement),
        Some(Token::While) => while_statement(state).map(Statement::WhileStatement),
        Some(Token::Try) => {
            unimplemented!("try statement")
            //state.proceed();
            //try_statement(state).map(Statement::TryStatement)
        },
        Some(Token::Throw) => {
            state.proceed();
            let res = expression(state).map(Statement::Throw);
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
                Ok(Statement::Return(None))
            } else {
                let e = expression(state)?;
                state.consume_1(Token::Semicolon)?;
                Ok(Statement::Return(Some(e)))
            }
        },
        _ => {
            let e = expression(state)?;
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::ExprStatement(e))
        }
    }
}

fn function_decl(state: &mut ParserState) -> Result<Function, String> {
    state.consume_1(Token::Function)?;
    let name = identifier(state)?;
    function_internal(state, Some(name))
}

fn declaration(state: &mut ParserState, kind: DeclarationKind) -> Result<Declaration, String> {
    repeated_elements(state, None, Token::Semicolon, &binding, false).map(|x| Declaration { kind, bindings: x })
}

pub fn binding(state: &mut ParserState) -> Result<Binding, String> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = expression(state)?;
            Ok(Binding::PatternBinding(pattern, expr))
        },
        _ => {
            let name = identifier(state)?;
            let expr = if state.try_proceed(Token::Equal) {
                Some(expression(state)?)
            } else {
                None
            };
            Ok(Binding::VariableBinding(name, expr))
        }
    }
}

pub fn block(state: &mut ParserState) -> Result<Block, String> {
    state.consume_1(Token::LeftBrace)?;

    let mut statements = vec![];
    while state.lookahead_1() != Some(Token::RightBrace) {
        statements.push(statement(state)?);
    }

    state.consume_1(Token::RightBrace)?;

    Ok(Block { statements })
}

fn if_statement(state: &mut ParserState) -> Result<IfStatement, String> {
    state.consume_1(Token::If)?;
    state.consume_1(Token::LeftParen)?;
    let condition = expression(state)?;
    state.consume_1(Token::RightParen)?;
    let consequent = block(state)?;

    let alternate = if state.try_proceed(Token::Else) {
        if state.try_proceed(Token::If) {
            Some(ElseArm::ElseIf(if_statement(state).map(Box::new)?))
        } else {
            Some(ElseArm::Body(block(state)?))
        }
    } else {
        None
    };

    Ok(IfStatement { condition, consequent, alternate })
}

pub fn while_statement(state: &mut ParserState) -> Result<WhileStatement, String> {
    state.consume_1(Token::While)?;
    state.consume_1(Token::LeftParen)?;
    let condition = expression(state)?;
    state.consume_1(Token::RightParen)?;
    let body = block(state)?;

    Ok(WhileStatement { condition, body })
}


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
pub fn expression(state: &mut ParserState) -> Result<Expr, String> {
    match state.lookahead_1() {
        // Function expression
        // function f(x) { return x + 1; }
        Some(Token::Function) => function_expr(state).map(|x| Expr::FunctionExpr(Box::new(x))),

        // Parenthesized expression or arrow function
        // (x + y)
        // (x, y) => x + y
        Some(Token::LeftParen) => arrow_or_paren_expr(state),

        _ => assign_or_cond_or_primary_expression(state),
    }
}

pub fn call_and_unary_op(state: &mut ParserState) -> Result<(Expr, bool), String> {
    // 1. UnaryOp fast path
    let mut preops = vec![];
    while let Ok(preop) = unary_op(state) {
        preops.push(preop)
    }

    // 2. Leftmost PrimaryExpression
    let expr = primary_expr(state)?;

    // 2-1. MemberPostOp fast path
    // If the leftmost node has CallPostOps(but without Call), it could be either an AssignExpr or a CondExpr(CallExpr).
    call_expr_internal(state, expr)
}

fn assign_or_cond_or_primary_expression(state: &mut ParserState) -> Result<Expr, String> {
    // Leftmost node of the assignment expression could be either PrimaryExpression or Variable.
    // Leftmost node of the conditional expression could be either UnaryOp(in case of UpdateExpr) or PrimaryExpression(in case of CallExpr).

    // We take the following approach:
    // 1. Try parsing a UnaryOp. If it succeeds, we know that we are parsing an CondExpr(with an UnaryExpr as the leftmost node). Power operator should not be parsed in this case. Return UnaryExpr parsing result.
    // 2. Try parsing a PrimaryExpression. If it succeeds, we know that we are parsing either an AssignExpr or a CondExpr(with a PrimaryExpression as the leftmost node).
    // 2-1. Try parsing a MemberPostOp. If it succeeds, we know that we are parsing either an AssignExpr or a CondExpr(with a CallExpr as the leftmost node). If it fails, check if the PrimaryExpression is a variable. If it is not, we are parsing a CondExpr. Return CondExpr parsing result. Otherwise, proceed to 3.
    // 3. Try parsing an AssignOp. If it succeeds, we know that we are parsing an AssignExpr. Coerce the primary expression to a LValue and return the AssignExpr parsing result. If it fails, we are parsing a CondExpr.
    // 4. Try parsing an ternary/binary operator. If it succeeds, we know that we are parsing a CondExpr. Return CondExpr parsing result.
    // 5. If we reach here, we are parsing a PrimaryExpression. Return the parsing result.
    // TODO: support quasi expression


    let (expr, only_member_post_op) = call_and_unary_op(state)?;

    if let Expr::UnaryExpr(_) = expr {
        // If the preops exist, we are parsing a CondExpr(UnaryExpr as leftmost). Apply preops to the leftmost node and jump to CondExpr.
        return cond_expr_with_leftmost_no_power(state, expr)
    } else if let Expr::CallExpr(_) = expr {
        // If a CallPostOp is found, it could be either an AssignExpr or a CondExpr(CallExpr).

        // continue
    } else if let Expr::Variable(_) = expr {
        // If the leftmost node is a variable, and there is no CallPostOp, it could be either an AssignExpr or a CondExpr(Variable).

        // continue
    } else {
        // Otherwise, it is a CondExpr(PrimaryExpression).

        return cond_expr_with_leftmost(state, expr)
    }

    // 3. AssignExpr parsing
    if let Some(op) = assign_op(state) {
        if let Some(lvalue) = expr.try_into_lvalue() {
            let right = expression(state)?;
            return Ok(Expr::Assignment(Box::new(Assignment ( lvalue, op, right ))));
        } else {
            return Err("Invalid left-hand side in assignment".to_string());
        }
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

// Lookaheads: 
// '(' => Paren
// '`' => Quasi // TODO
// '[' => DataStructure/Array
// '{' => DataStructure/Record
// '"' => DataStructure/DataLiteral/String
// 'n' ull => DataStructure/DataLiteral/Null
// 't' rue => DataStructure/DataLiteral/True
// 'f' alse => DataStructure/DataLiteral/False
// 'u' ndefined => DataStructure/DataLiteral/Undefined
// '0'..'9' => DataStructure/DataLiteral/Number
// 'f' unction => FunctionExpr
// ascii => use_variable
// did I miss anything? anything else????
pub fn primary_expr(state: &mut ParserState) -> Result<Expr, String> {
    match state.lookahead_1() {
        Some(Token::LeftParen) => {
            state.proceed();
            let e = expression(state)?;
            state.consume_1(Token::RightParen)?;
            Ok(Expr::ParenedExpr(Box::new(e)))
        },
        Some(Token::QuasiQuote) => unimplemented!("QuasiExpr not implemented"),
        Some(Token::LeftBracket) => array(state).map(Expr::Array),
        Some(Token::LeftBrace) => record(state).map(Expr::Record),
        Some(Token::String(s)) => state.proceed_then(Expr::DataLiteral(DataLiteral::String(s))),
        Some(Token::Number(n)) => state.proceed_then(Expr::DataLiteral(DataLiteral::Number(n))),
        Some(Token::Null) => state.proceed_then(Expr::DataLiteral(DataLiteral::Null)),
        Some(Token::True) => state.proceed_then(Expr::DataLiteral(DataLiteral::True)),
        Some(Token::False) => state.proceed_then(Expr::DataLiteral(DataLiteral::False)),
        Some(Token::Undefined) => state.proceed_then(Expr::DataLiteral(DataLiteral::Undefined)),
        Some(Token::Bigint(b)) => state.proceed_then(Expr::DataLiteral(DataLiteral::Bigint(b))),
        Some(Token::Function) => function_expr(state).map(|x| Expr::FunctionExpr(Box::new(x))),
        _ => use_variable(state).map(|x| Expr::Variable(x)),
    }
}

fn function_expr(state: &mut ParserState) -> Result<Function, String> {
    state.consume_1(Token::Function)?;
    let name = if let Some(Token::Identifier(name)) = state.lookahead_1() {
        state.proceed();
        Some(name)
    } else {
        None
    };
    function_internal(state, name)
}

fn function_internal(state: &mut ParserState, name: Option<String>) -> Result<Function, String> {
    let args = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;
    // TODO: spread parameter can only come at the end
    match state.lookahead_1() {
        Some(Token::LeftBrace) => Ok(Function(name, args, None/*TODO */, block(state)?)),
        _ => unimplemented!("Function expressions without block body are not supported yet")
    }
}

fn arrow_or_paren_expr(state: &mut ParserState) -> Result<Expr, String> {
    let args = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;
    if state.try_proceed(Token::FatArrow) {
        match state.lookahead_1() {
            Some(Token::LeftBrace) => return Ok(Expr::ArrowFunc(Box::new(Function(None, args, None/*TODO */, block(state)?)))),
            _ => unimplemented!("Arrow functions without block body are not supported yet")
        }    
    } else {
        match &args[..] {
            [p] => match p {
                Pattern::Variable(x, ann) => {
                    if ann.is_some() {
                        unimplemented!("Type annotations should not be in parenthesized expressions")
                    }
                    Ok(Expr::Variable(x.to_string()))
                },
                Pattern::ArrayPattern(xs, ann) => {
                    unimplemented!("array literal in parenthesized expressions is not supported yet")
                },
                Pattern::RecordPattern(xs, ann) => {
                    unimplemented!("record literal in parenthesized expressions is not supported yet")
                },
                _ => Err("=> expected".to_string()),
            }
            _ => Err("Grouped expressions are not valid syntax in Jessie".to_string()),
        }
    }
}

pub fn array(state: &mut ParserState) -> Result<Array, String> {
    let elements = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &element, true)?;
    Ok(Array(elements))
}

pub fn element(state: &mut ParserState) -> Result<Element, String> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Ok(Element::Spread(expr))
    } else {
        let expr = expression(state)?;
        Ok(Element::Expr(expr))
    }
}

pub fn record(state: &mut ParserState) -> Result<Record, String> {
    let props = repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_def, true)?;
    Ok(Record(props))
}

pub fn pure_prop_def(state: &mut ParserState) -> Result<PropDef, String> {
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
            Ok(PropDef::Shorthand(prop_name))
        }
    }
}

pub fn prop_def(state: &mut ParserState) -> Result<PropDef, String> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Ok(PropDef::Spread(expr))
    }
    else if state.lookahead_1() == Some(Token::QuasiQuote) {
        Err("QuasiExpr not implemented".to_string())
    } else {
        let prop_name = prop_name(state)?;
        if state.try_proceed(Token::Colon) {
            let expr = expression(state)?;
            Ok(PropDef::KeyValue(prop_name, expr))
        } else {
            Ok(PropDef::Shorthand(prop_name))
        }
    }
}

pub fn arg(state: &mut ParserState) -> Result<Arg, String> {
    // TODO: spread parameter can only come at the end
    if state.try_proceed(Token::DotDotDot) {
        let e = expression(state)?;
        Ok(Arg::Spread(e))
    } else {
        let expr = expression(state)?;
        Ok(Arg::Expr(expr))
    }
}

///////////////////////////
// Arithmetic expressions
// From this points, all `_with_leftmost` functions are expected to take the `left` argument already parsed as an CallExpr.

pub fn cond_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
    if state.try_proceed(Token::Question) {
        let expr1 = expression(state)?;
        state.consume_1(Token::Colon)?;
        let expr2 = expression(state)?;
        Ok(Expr::CondExpr(Box::new(CondExpr(result, expr1, expr2))))
    } else {
        Ok(result)
    }
}

pub fn cond_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let or_else_expr = or_else_expr_with_leftmost(state, left)?;
    cond_expr_internal(state, or_else_expr)
}

pub fn cond_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let or_else_expr = or_else_expr_with_leftmost_no_power(state, left)?;
    cond_expr_internal(state, or_else_expr)
}

fn or_else_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
    while state.try_proceed(Token::BarBar) {
        let and_then_expr2 = and_then_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Or, result, and_then_expr2)))
    }
    Ok(result)
}

pub fn or_else_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = and_then_expr_with_leftmost(state, left)?;
    or_else_expr_internal(state, result)
}

pub fn or_else_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = and_then_expr_with_leftmost_no_power(state, left)?;
    or_else_expr_internal(state, result)
}

fn and_then_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
    while state.try_proceed(Token::AmpAmp) {
        let eager_expr2 = eager_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::And, result, eager_expr2)))
    }
    Ok(result)
}

pub fn and_then_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = eager_expr(state)?;
    and_then_expr_internal(state, result)
}

pub fn and_then_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = eager_expr_with_leftmost(state, left)?;
    and_then_expr_internal(state, result)
}

pub fn and_then_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = eager_expr_with_leftmost_no_power(state, left)?;
    and_then_expr_internal(state, result)
}

pub fn eager_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
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
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseAnd, result, shift_expr(state)?)))
            },
            Token::Bar => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseOr, result, shift_expr(state)?)))
            },
            Token::Caret => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseXor, result, shift_expr(state)?)))
            }, 
            _ => break,
        }
    }

    Ok(result)
}

pub fn eager_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = shift_expr(state)?;
    eager_expr_internal(state, result)
}

pub fn eager_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = shift_expr_with_leftmost(state, left)?;
    eager_expr_internal(state, result)
}

pub fn eager_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = shift_expr_with_leftmost_no_power(state, left)?;
    eager_expr_internal(state, result)
}

fn shift_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::LAngleLAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseLeftShift, result, add_expr(state)?)))
            },
            Token::RAngleRAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseRightShift, result, add_expr(state)?)))
            },
            Token::RAngleRAngleRAngle => result = {
                state.proceed();
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseUnsignedRightShift, result, add_expr(state)?)))
            },
            _ => break,
        }
    }

    Ok(result)
}

pub fn shift_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = add_expr(state)?;
    shift_expr_internal(state, result)
}

pub fn shift_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = add_expr_with_leftmost(state, left)?;
    shift_expr_internal(state, result)
}

pub fn shift_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = add_expr_with_leftmost_no_power(state, left)?;
    shift_expr_internal(state, result)
}

pub fn add_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
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

pub fn add_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = mult_expr(state)?;
    add_expr_internal(state, result)
}

pub fn add_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = mult_expr_with_leftmost(state, left)?;
    add_expr_internal(state, result)
}

pub fn add_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = mult_expr_with_leftmost_no_power(state, left)?;
    add_expr_internal(state, result)
}

pub fn mult_expr_internal(state: &mut ParserState, mut result: Expr) -> Result<Expr, String> {
    let mut result = pow_expr(state)?;
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

pub fn mult_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = pow_expr(state)?;
    mult_expr_internal(state, result)
}

pub fn mult_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = pow_expr_with_leftmost(state, left)?;
    mult_expr_internal(state, result)
}

pub fn mult_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    let mut result = pow_expr_with_leftmost_no_power(state, left)?;
    mult_expr_internal(state, result)
}

fn pow_expr_internal(state: &mut ParserState, result: Expr) -> Result<Expr, String> {
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

fn pow_expr(state: &mut ParserState) -> Result<Expr, String> {
    let (left, _) = call_and_unary_op(state)?;
    if let Expr::UnaryExpr(_) = left {
        // Not a UnaryExpression, so we can parse the power expression.
        pow_expr_with_leftmost(state, left)
    } else {
        // Is a UnaryExpression, so we cannot parse the power expression.
        pow_expr_with_leftmost_no_power(state, left)
    }
}

fn pow_expr_with_leftmost(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    // the leftmost expression is already parsed as an UpdateExpression, and the power operator can come after.
    pow_expr_internal(state, left)
}

fn pow_expr_with_leftmost_no_power(state: &mut ParserState, left: Expr) -> Result<Expr, String> {
    // the leftmost expression is already parsed as a UnaryExpression, and the power operator cannot come after.
    // so we can skip the power expression parsing.
    if let Some(Token::AsteriskAsterisk) = state.lookahead_1() {
        Err("Unexpected token **".to_string())
    } else {
        Ok(left)
    }
}

fn call_post_op(state: &mut ParserState) -> Result<CallPostOp, String> {
    match state.lookahead_1() {
        Some(Token::LeftParen) => repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &arg, true).map(CallPostOp::Call),
        Some(Token::LeftBracket) => enclosed_element(state, Token::LeftBracket, Token::RightBracket, &expression).map(|x| CallPostOp::MemberPostOp(MemberPostOp::Index(x))),
        Some(Token::Dot) => {
            state.proceed();
            let ident = identifier(state)?;
            Ok(CallPostOp::MemberPostOp(MemberPostOp::Member(ident)))
        },
        _ => Err("Unexpected token".to_string()),
    }
}

fn call_expr_internal(state: &mut ParserState, mut expr: Expr) -> Result<(Expr, bool), String> { 
    let mut only_member_post_op = true;

    while let Ok(post_op) = call_post_op(state) {
        match post_op {
            CallPostOp::Call(_) => { only_member_post_op = false },
            CallPostOp::MemberPostOp(_) => {},
        }
        expr = Expr::CallExpr(Box::new(CallExpr { expr, post_op }));
    }

    Ok((expr, only_member_post_op))
}

pub fn call_expr_with_leftmost(state: &mut ParserState, mut expr: Expr) -> Result<Expr, String> {
    // with_leftmost functions take the leftmost expression already parsed as a CallExpression. We can just return it.

    // I believe rust compiler will take care of it
    Ok(expr)
} 

pub fn call_expr(state: &mut ParserState) -> Result<Expr, String> {
    let expr = primary_expr(state)?;
    let (result, _) = call_expr_internal(state, expr)?;
    Ok(result)
}



pub fn unary_op(state: &mut ParserState) -> Result<UnaryOp, String> {
    match state.lookahead_1() {
        Some(Token::Plus) => state.proceed_then(UnaryOp::Pos),
        Some(Token::Minus) => state.proceed_then(UnaryOp::Neg),
        Some(Token::Bang) => state.proceed_then(UnaryOp::Not),
        Some(Token::Tilde) => state.proceed_then(UnaryOp::BitNot),
        Some(Token::TypeOf) => state.proceed_then(UnaryOp::TypeOf),
        _ => Err("Unexpected token".to_string()),
    }
}

fn lookahead_operator(state: &mut ParserState) -> bool {
    match state.lookahead_1() {
        Some(Token::Question) |
        Some(Token::Plus) |
        Some(Token::Minus) |
        Some(Token::Asterisk) |
        Some(Token::Slash) |
        Some(Token::Percent) |
        Some(Token::AsteriskAsterisk) |
        Some(Token::Ampersand) |
        Some(Token::Bar) |
        Some(Token::Caret) |
        Some(Token::LAngle) |
        Some(Token::RAngle) |
        Some(Token::LAngleEqual) |
        Some(Token::RAngleEqual) |
        Some(Token::EqualEqualEqual) |
        Some(Token::BangEqualEqual) |
        Some(Token::LAngleLAngle) |
        Some(Token::RAngleRAngle) |
        Some(Token::RAngleRAngleRAngle) |
        Some(Token::AmpAmp) |
        Some(Token::BarBar) |
        Some(Token::QuestionQuestion) => true,
        _ => false,
    }
}

///////////////////////////
// Basic components

pub fn identifier(state: &mut ParserState) -> Result<String, String> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(s)
        },
        _ => Err("Unexpected token".to_string()),
    }
}

pub fn def_variable(state: &mut ParserState) -> Result<String, String> {
    let ident = identifier(state)?;
    // let type_ann = optional_type_ann(state)?;
    Ok(ident)
}

pub fn use_variable(state: &mut ParserState) -> Result<String, String> {
    let ident = identifier(state)?;
    Ok(ident)
}

pub fn optional_type_ann(state: &mut ParserState) -> Result<Option<TypeAnn>, String> {
    Ok(None)
}

pub fn prop_name(state: &mut ParserState) -> Result<PropName, String> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(PropName::Ident(s))
        },
        Some(Token::String(s)) => {
            state.proceed();
            Ok(PropName::String(s))
        },
        Some(Token::Number(s)) => {
            state.proceed();
            Ok(PropName::Number(s))
        },
        _ => Err("Unexpected token".to_string()),
    }
}