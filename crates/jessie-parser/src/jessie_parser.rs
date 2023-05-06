use std::rc::Rc;

use crate::parser::{self, ArrayLike, err_expected, err_invalid, err_unimplemented};
use crate::lexer::{self, Token, VecToken, repeated_elements, enclosed_element};
use jessie_ast::*;

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

// stuffs to care about:
// https://github.com/mozilla-spidermonkey/jsparagus/blob/master/js-quirks.md#readme

///////////////////////
// Module

pub fn module_body(state: &mut ParserState) -> Result<ModuleBody, ParserError> {
    let mut items = Vec::new();

    while let Some(_) = state.lookahead_1() {
        items.push(module_item(state)?);
    }

    Ok(ModuleBody(items))
}

pub fn module_item(state: &mut ParserState) -> Result<ModuleItem, ParserError> {
    module_decl(state).map(ModuleItem::ModuleDecl) // TODO
}

pub fn module_decl(state: &mut ParserState) -> Result<ModuleDecl, ParserError> {
    state.consume_1(Token::Const)?;
    repeated_elements(state, None, Token::Semicolon, &module_binding, false).map(ModuleDecl)
}

pub fn hardened_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    Ok(expression(state)?) // TODO
}

pub fn module_binding(state: &mut ParserState) -> Result<ModuleBinding, ParserError> {
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

pub fn binding_pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &param, false).map(|x| Pattern::ArrayPattern(x, None/*TODO */)),
        Some(Token::LeftBrace) => repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_param, false).map(|x| Pattern::RecordPattern(x, None/*TODO */)),
        c => err_expected("binding pattern", c),
    }
}

// only parses original "pattern" rule from Jessica, not the entire variants of enum Pattern.
// consider changing the name to binding_or_ident_pattern or something...
pub fn pattern(state: &mut ParserState) -> Result<Pattern, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::LeftBrace) => binding_pattern(state),
        Some(Token::Comma) | Some(Token::RightBracket) => Ok(Pattern::Hole), // Not sure if its the right way...
        _ => // data_literal(state).map(|x| Pattern::DataLiteral(x)).or_else(|_| {
            def_variable(state).map(|x| Pattern::Variable(x, None/*TODO */))
        //}),
    }
}

pub fn param(state: &mut ParserState) -> Result<Pattern, ParserError> {
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

fn prop_param(state: &mut ParserState) -> Result<PropParam, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| PropParam::Rest(x))
    }

    let key = def_variable(state)?; // def or use XXX

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
pub fn statement(state: &mut ParserState) -> Result<Statement, ParserError> {
    // putting whitespace in consumes is a hack, need to fix later
    match state.lookahead_1() {
        Some(Token::LeftBrace) => block(state).map(|x| Statement::Block(Box::new(x))), // TODO: implement statement level record literal?
        Some(Token::Const) => {
            state.proceed();
            let decl = Rc::new(repeated_elements(state, None, Token::Semicolon, &binding, false).map(Declaration::Const)?);
            state.scope.settle_declaration(decl.clone());
            Ok(Statement::Declaration(decl))
        },
        Some(Token::Let) => {
            state.proceed();
            let decl = Rc::new(repeated_elements(state, None, Token::Semicolon, &binding, false).map(Declaration::Let)?);
            state.scope.settle_declaration(decl.clone());
            Ok(Statement::Declaration(decl))
        },
        Some(Token::Function) => function_decl(state).map(Statement::Declaration),
        Some(Token::If) => if_statement(state).map(|x| Statement::IfStatement(Box::new(x))),
        Some(Token::While) => while_statement(state).map(|x| Statement::WhileStatement(Box::new(x))),
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

fn function_decl(state: &mut ParserState) -> Result<Rc<Declaration>, ParserError> {
    state.consume_1(Token::Function)?;
    let name = def_variable(state)?;
    function_internal(state, Some(name)).map(|x| Rc::new(Declaration::Function(Box::new(x))))
}

pub fn binding(state: &mut ParserState) -> Result<Binding, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = expression(state)?;
            Ok(Binding::PatternBinding(pattern, expr))
        },
        _ => {
            let name = def_variable(state)?;
            let expr = if state.try_proceed(Token::Equal) {
                Some(expression(state)?)
            } else {
                None
            };
            Ok(Binding::VariableBinding(name, expr))
        }
    }
}

pub fn block(state: &mut ParserState) -> Result<Block, ParserError> {
    let parent_scope = state.enter_block_scope();

    let statements = block_raw(state)?;

    let scope = state.exit_block_scope(parent_scope);

    Ok(Block { statements, scope })
}

fn block_raw(state: &mut ParserState) -> Result<Vec<Statement>, ParserError> {
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
            Some(ElseArm::ElseIf(if_statement(state).map(Box::new)?))
        } else {
            Some(ElseArm::Body(block(state)?))
        }
    } else {
        None
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
        
        Some(Token::Function) => function_expr(state).map(|x| Expr::FunctionExpr(Box::new(x))),

        // Parenthesized expression or arrow function
        // (x + y)
        // (x, y) => x + y
        Some(Token::LeftParen) => {
            let expr = arrow_or_paren_expr(state)?;
            // if the expression is parenthesized expression(a primary expression), 
            // it can be a leftmost expression for a CondExpr.
            if let Expr::ParenedExpr(_) = expr {
                cond_expr_with_leftmost(state, expr)
            } else {
                Ok(expr)
            }
        }

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



fn assign_or_cond_or_primary_expression(state: &mut ParserState) -> Result<Expr, ParserError> {
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
        Expr::DataLiteral(_) | Expr::FunctionExpr(_) => {
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
        return Ok(Expr::Assignment(Box::new(Assignment ( lvalue, op, right ))));
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

fn function_expr(state: &mut ParserState) -> Result<Function, ParserError> {
    state.consume_1(Token::Function)?;
    let name = def_variable(state).ok();
    function_internal(state, name)
}

fn function_internal(state: &mut ParserState, name: Option<DefVariable>) -> Result<Function, ParserError> {
    let parent_scope = state.enter_block_scope();
    
    let parameters = Rc::new(Declaration::Parameters(repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?));
    // TODO: spread parameter can only come at the end
    println!("function_internal");

    state.scope.settle_declaration(parameters.clone());

    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            let statements = block_raw(state)?;
            let scope = state.exit_block_scope(parent_scope);
            let func = Function {
                name,
                parameters,
                typeann: None,
                statements,
                expression: None,
                scope,
            };
            Ok(func)
        },
        c => err_expected(&"a function body", c),
    } 
}

enum CoverArrow {
    PossiblyArrowParameter(Expr),
    NotArrowParameter(Expr),
}

fn expression_or_destruction(state: &mut ParserState) -> Result<CoverArrow, ParserError> {
    // TODO: implement destructing pattern for arrow arguments
    // Array or object pattern cannot come in the lefthand of an assignment,
    // so we can parse a possibly destructing object / array first,
    // and feed it to cond_expr_with_leftmost if an operator follows.
    expression(state).map(CoverArrow::NotArrowParameter)
}

fn assignment_or_parameter_or_optional(state: &mut ParserState) -> Result<CoverArrow, ParserError> {
    let expr = assign_or_cond_or_primary_expression(state)?;

    match expr {
        // Expression can be a parameter if it is a pure variable
        Expr::Variable(_) => Ok(CoverArrow::PossiblyArrowParameter(expr)),

        // Expression can be a parameter if it is strictly a form of variable = expression
        Expr::Assignment(ref assign) => if let Assignment(LValue::Variable(_), AssignOp::Assign, _) = **assign {
            Ok(CoverArrow::PossiblyArrowParameter(expr))
        } else {
            Ok(CoverArrow::NotArrowParameter(expr))
        },

        // Otherwise, it is an expression.
        _ => Ok(CoverArrow::NotArrowParameter(expr)), 
    }
}

// parses a single CPEAAPL argument(not the whole list)
// returns the expression
fn cpeaapl(state: &mut ParserState) -> Result<CoverArrow, ParserError> {
    match state.lookahead_1() {
        // Destructuring pattern or Expressions
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => expression_or_destruction(state),

        // Default pattern or Assignment expressions
        Some(Token::Identifier(_)) => assignment_or_parameter_or_optional(state),

        // No other cases are valid for arrow argument
        _ => expression(state).map(CoverArrow::NotArrowParameter),
    }
}

fn arrow_function_body(state: &mut ParserState) -> Result<BlockOrExpr, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) => {
            state.proceed();
            block_raw(state).map(BlockOrExpr::Block)
        },
        _ => {
            expression(state).map(BlockOrExpr::Expr)
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
fn arrow_or_paren_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    // Split into three cases:
    // () => Parenthesized expression cannot be empty, nullary arrow function.
    // (CPEAAPL) => Required to cover both cases only in unary case.
    // (param, ...) => No group expression in Jessie, n-ary arrow function.

    state.proceed(); // consume left paren
    

    let parent_scope = state.enter_block_scope();
    
    // nullary arrow function
    if state.try_proceed(Token::RightParen) {
        state.consume_1(Token::FatArrow)?;

        let body = arrow_function_body(state)?;
        let scope = state.exit_block_scope(parent_scope);

        return Ok(Expr::ArrowFunc(Box::new(Function::from_body(None, Rc::new(Declaration::Parameters(vec![])), None, body, scope))))
    }

    // unary spread parameter
    if state.try_proceed(Token::DotDotDot) {
        let arg = param(state)?;
        let parameters = Rc::new(Declaration::Parameters(vec![Pattern::Rest(Box::new(arg), None)]));
        state.scope.settle_declaration(parameters.clone());
        let body = arrow_function_body(state)?;
        let scope = state.exit_block_scope(parent_scope);
        return Ok(Expr::ArrowFunc(Box::new(Function::from_body(None, parameters, None, body, scope))))
    }

    // try parse a single CPEAAPL first
    match cpeaapl(state)? {
        // Possibly a unary arrow parameter.
        CoverArrow::PossiblyArrowParameter(first) => {
            match state.lookahead_1() {
                // If a comma follows, it is an n-ary arrow function.
                // TODO: spread element should come only at the last 
                Some(Token::Comma) => {
                    state.proceed();
                    let rest = repeated_elements(state, None, Token::RightParen, &param, true)?;
                    let mut params_vec = vec![first.into()];
                    params_vec.extend(rest); // TODO: optimize
                    let parameters = Rc::new(Declaration::Parameters(params_vec));
                    state.scope.settle_declaration(parameters.clone());
                    let body = arrow_function_body(state)?;
                    let scope = state.exit_block_scope(parent_scope);
                    return Ok(Expr::ArrowFunc(Box::new(Function::from_body(None, parameters, None, body, scope))))

                },
                // If a right paren follows, try parse a fat arrow.
                Some(Token::RightParen) => {
                    state.proceed();
                    match state.lookahead_1() {
                        // If a fat arrow follows, it is a unary arrow function.
                        Some(Token::FatArrow) => {
                            state.proceed();
                            let parameters = Rc::new(Declaration::Parameters(vec![first.into()]));
                            state.scope.settle_declaration(parameters.clone());
                            let body = arrow_function_body(state)?;
                            let scope = state.exit_block_scope(parent_scope);
                            return Ok(Expr::ArrowFunc(Box::new(Function::from_body(None, parameters, None, body, scope))))
                        },
                        // Otherwise, it is a parenthesized expression.
                        _ => {
                            state.exit_merge_block_scope(parent_scope);
                            return Ok(Expr::ParenedExpr(Box::new(first)))
                        } 
                    }
                }
                // No other token should follow after.
                c => err_expected("end of the parenthesized expression or another arrow parameter", c),
            }
        }

        // Not an arrow parameter, should be a parenthesized expression.
        CoverArrow::NotArrowParameter(expr) => {
            state.consume_1(Token::RightParen)?;
            return Ok(Expr::ParenedExpr(Box::new(expr)))
        }
    }
}

pub fn array(state: &mut ParserState) -> Result<Array, ParserError> {
    let elements = repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &element, true)?;
    Ok(Array(elements))
}

pub fn element(state: &mut ParserState) -> Result<Element, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Ok(Element::Spread(expr))
    } else {
        let expr = expression(state)?;
        Ok(Element::Expr(expr))
    }
}

pub fn record(state: &mut ParserState) -> Result<Record, ParserError> {
    let props = repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_def, true)?;
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

pub fn prop_def(state: &mut ParserState) -> Result<PropDef, ParserError> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expression(state)?;
        Ok(PropDef::Spread(expr))
    }
    else if state.lookahead_1() == Some(Token::QuasiQuote) {
        return err_unimplemented("quasiquote")
    } else {
        let prop_name = prop_name(state)?;
        if state.try_proceed(Token::Colon) {
            let expr = expression(state)?;
            Ok(PropDef::KeyValue(prop_name, expr))
        } else if let PropName::Ident(ident) = prop_name {
            let shorthand = use_variable_with_parsed(state, ident.clone());
            Ok(PropDef::Shorthand(shorthand))
        } else {
            err_expected("colon or equal sign", state.lookahead_1())
        }
    }
}

pub fn arg(state: &mut ParserState) -> Result<Arg, ParserError> {
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
        return err_expected("no power operator", Some(Token::AsteriskAsterisk))
    } else {
        Ok(left)
    }
}

fn call_post_op(state: &mut ParserState) -> Result<CallPostOp, ParserError> {
    match state.lookahead_1() {
        Some(Token::LeftParen) => repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &arg, true).map(CallPostOp::Call),
        Some(Token::LeftBracket) => enclosed_element(state, Token::LeftBracket, Token::RightBracket, &expression).map(|x| CallPostOp::MemberPostOp(MemberPostOp::Index(x))),
        Some(Token::Dot) => {
            state.proceed();
            let ident = identifier(state)?;
            Ok(CallPostOp::MemberPostOp(MemberPostOp::Member(ident)))
        },
        c => err_expected("index, member, or call operator", c),
    }
}

fn call_expr_internal(state: &mut ParserState, mut expr: Expr) -> Result<(Expr, bool), ParserError> { 
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
        c => err_expected("unary operator", c),
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

pub fn identifier(state: &mut ParserState) -> Result<String, ParserError> {
    match state.lookahead_1() {
        Some(Token::Identifier(s)) => {
            state.proceed();
            Ok(s)
        },
        c => err_expected("identifier", c),
    }
}

pub fn def_variable(state: &mut ParserState) -> Result<DefVariable, ParserError> {
    let ident = identifier(state)?;
    let var = state.scope.def_variable(&ident)?;
    // let type_ann = optional_type_ann(state)?;
    Ok(var)
}

pub fn use_variable(state: &mut ParserState) -> Result<UseVariable, ParserError> {
    let ident = identifier(state)?;
    let var = state.scope.use_variable(&ident);
    println!("use variable {:?}", state);
    Ok(var)
}

pub fn use_variable_with_parsed(state: &mut ParserState, ident: String) -> UseVariable {
    let var = state.scope.use_variable(&ident);
    println!("use variable {:?}", state);
    var 
}

pub fn optional_type_ann(state: &mut ParserState) -> Result<Option<TypeAnn>, ParserError> {
    Ok(None)
}

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
        Some(Token::Number(s)) => {
            state.proceed();
            Ok(PropName::Number(s))
        },
        c => err_expected("property name", c),
    }
}